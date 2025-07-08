pub mod registry;
pub mod system_bi;
pub mod system_three;
pub mod system_drs;

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::capture::HttpPacket;
use crate::auth::events;
use log::{info, warn, debug};
use regex::Regex;

/// 系统认证接口
pub trait SystemAuth {
    /// 获取系统ID
    fn system_id(&self) -> &str;
    
    /// 获取系统名称
    fn system_name(&self) -> &str;
    
    /// 处理HTTP数据包，尝试提取token（核心方法）
    fn process_http_request(&mut self, packet: &HttpPacket) -> Result<()>;
    
    /// 处理获取到的token
    fn handle_token(&mut self, token: &str, acquired_at: u64, expires_at: u64) -> Result<()>;
    
    /// 检查token是否有效
    fn is_token_valid(&self) -> bool;
    
    /// 获取当前token
    fn get_current_token(&self) -> Option<&str>;
    
    /// 获取token状态
    fn get_token_info(&self) -> TokenInfo;
    
    /// 清除token
    fn clear_token(&mut self);
    
}

/// Token验证器接口
pub trait TokenValidator: Send + Sync + std::fmt::Debug {
    fn validate(&self, token: &str) -> Result<()>;
}

/// 系统配置
#[derive(Debug)]
pub struct SystemConfig {
    /// 系统ID
    pub system_id: String,
    /// 系统名称
    pub system_name: String,
    /// URL匹配正则模式
    pub url_pattern: String,
    /// Token所在的header名称
    pub header_name: String,
    /// Token提取正则模式
    pub token_pattern: String,
    /// Token过期时间（秒）
    pub expires_duration: u64,
    /// Token验证器
    pub validator: Box<dyn TokenValidator>,
}

/// 基础系统实现
pub struct BaseSystem {
    config: SystemConfig,
    token_info: TokenInfo,
}

impl BaseSystem {
    /// 创建新的基础系统实例
    pub fn new(config: SystemConfig) -> Self {
        debug!("🏗️ 创建系统实例: {} ({})", config.system_id, config.system_name);
        Self {
            config,
            token_info: TokenInfo::new(),
        }
    }

    /// 检查URL是否匹配
    fn matches_url(&self, url: &str) -> bool {
        debug!("🔍 系统[{}]检查URL匹配: {}", self.config.system_id, url);
        
        if let Ok(regex) = Regex::new(&self.config.url_pattern) {
            let matches = regex.is_match(url);
            debug!("📋 系统[{}] URL匹配结果: {} (模式: {})", 
                   self.config.system_id, matches, self.config.url_pattern);
            matches
        } else {
            warn!("❌ 系统[{}] URL匹配正则表达式编译失败: {}", 
                  self.config.system_id, self.config.url_pattern);
            false
        }
    }

    /// 从HTTP数据包中提取token
    fn extract_token_from_request(&self, packet: &HttpPacket) -> Option<String> {
        debug!("🔎 系统[{}]开始提取token，Headers数量: {}", 
               self.config.system_id, packet.headers.len());
        
        // 查找指定的header
        let auth_header = packet.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(&self.config.header_name))
            .map(|(_, value)| value);
            
        let auth_header = match auth_header {
            Some(header) => {
                debug!("📋 系统[{}]找到{}header: {}...", 
                       self.config.system_id, self.config.header_name,
                       &header[..header.len().min(20)]);
                header
            }
            None => {
                debug!("❌ 系统[{}]未找到{}header", 
                       self.config.system_id, self.config.header_name);
                return None;
            }
        };

        // 使用正则提取token
        if let Ok(regex) = Regex::new(&self.config.token_pattern) {
            if let Some(captures) = regex.captures(auth_header) {
                if let Some(token_match) = captures.get(1) {
                    let token = token_match.as_str().to_string();
                    debug!("✅ 系统[{}]成功提取token，长度: {}", 
                           self.config.system_id, token.len());
                    return Some(token);
                }
            }
        }

        debug!("❌ 系统[{}]token提取失败，header值不匹配模式: {}", 
               self.config.system_id, self.config.token_pattern);
        None
    }
}

impl SystemAuth for BaseSystem {
    fn system_id(&self) -> &str {
        &self.config.system_id
    }
    
    fn system_name(&self) -> &str {
        &self.config.system_name
    }
    
    fn process_http_request(&mut self, packet: &HttpPacket) -> Result<()> {
        // 只处理HTTP请求，跳过响应
        if packet.packet_type != "request" {
            debug!("⏭️ 系统[{}]跳过HTTP{}处理", self.config.system_id, packet.packet_type);
            return Ok(());
        }
        
        let url = build_url(packet);
        debug!("🎯 系统[{}]开始处理HTTP请求: {} {}", 
               self.config.system_id, 
               packet.method.as_ref().unwrap_or(&"UNKNOWN".to_string()), 
               url);
        
        // 检查URL是否匹配
        if !self.matches_url(&url) {
            debug!("⏭️ 系统[{}]跳过处理：URL不匹配", self.config.system_id);
            return Ok(());
        }
        
        info!("🎯 系统[{}]检测到匹配的URL: {}", self.config.system_id, url);
        
        // 提取token
        let token = match self.extract_token_from_request(packet) {
            Some(token) => {
                debug!("📨 系统[{}]成功提取到token", self.config.system_id);
                token
            }
            None => {
                debug!("📭 系统[{}]未找到有效的{}token", 
                       self.config.system_id, self.config.header_name);
                return Ok(());
            }
        };
        
        // 验证token
        if let Err(e) = self.config.validator.validate(&token) {
            warn!("❌ 系统[{}]token验证失败: {}", self.config.system_id, e);
            
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            events::emit_token_failed(
                self.system_id().to_string(),
                self.system_name().to_string(),
                e.to_string(),
                now,
            );
            return Ok(());
        }
        
        // 检查是否是新token
        let is_new_token = if let Some(current_token) = self.get_current_token() {
            let is_new = current_token != token;
            debug!("🔄 系统[{}]token比较: 是否为新token = {}", 
                   self.config.system_id, is_new);
            is_new
        } else {
            debug!("🆕 系统[{}]首次获取token", self.config.system_id);
            true
        };
        
        if is_new_token {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let expires_at = now + self.config.expires_duration;
            debug!("⏰ 系统[{}]设置token过期时间: {} ({}秒后)", 
                   self.config.system_id, expires_at, self.config.expires_duration);
            
            if let Err(e) = self.handle_token(&token, now, expires_at) {
                warn!("❌ 系统[{}]处理token失败: {}", self.config.system_id, e);
                return Ok(());
            }
            
            info!("🎉 系统[{}]新token处理成功", self.config.system_id);
            events::emit_token_acquired(
                self.system_id().to_string(),
                self.system_name().to_string(),
                token,
                now,
                expires_at,
                url,
            );
        } else {
            debug!("🔄 系统[{}]token未变化，跳过更新", self.config.system_id);
        }
        
        Ok(())
    }
    
    fn handle_token(&mut self, token: &str, acquired_at: u64, expires_at: u64) -> Result<()> {
        info!("🎯 系统[{}]处理新token，长度: {}，有效期: {}秒", 
              self.config.system_id, token.len(), expires_at - acquired_at);
        
        self.token_info.update_token(token.to_string(), acquired_at, expires_at);
        
        info!("✅ 系统[{}]token更新成功，过期时间: {}", 
              self.config.system_id, expires_at);
        Ok(())
    }
    
    fn is_token_valid(&self) -> bool {
        let valid = self.token_info.is_valid && !self.token_info.is_expired();
        debug!("🔍 系统[{}]token有效性检查: {}", self.config.system_id, valid);
        valid
    }
    
    fn get_current_token(&self) -> Option<&str> {
        if self.is_token_valid() {
            debug!("✅ 系统[{}]返回有效token", self.config.system_id);
            self.token_info.token.as_deref()
        } else {
            debug!("❌ 系统[{}]token无效，返回None", self.config.system_id);
            None
        }
    }
    
    fn get_token_info(&self) -> TokenInfo {
        debug!("📊 系统[{}]返回token信息", self.config.system_id);
        self.token_info.clone()
    }
    
    fn clear_token(&mut self) {
        warn!("🗑️ 清除系统[{}]token", self.config.system_id);
        self.token_info.token = None;
        self.token_info.is_valid = false;
        self.token_info.acquired_at = None;
        self.token_info.expires_at = None;
        debug!("✅ 系统[{}]token已清除", self.config.system_id);
    }
}

/// Token信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token: Option<String>,
    pub acquired_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub is_valid: bool,
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self {
            token: None,
            acquired_at: None,
            expires_at: None,
            is_valid: false,
        }
    }
}

impl TokenInfo {
    /// 创建新的token信息
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 更新token
    pub fn update_token(&mut self, token: String, acquired_at: u64, expires_at: u64) {
        self.token = Some(token);
        self.acquired_at = Some(acquired_at);
        self.expires_at = Some(expires_at);
        self.is_valid = true;
    }
    
    /// 检查是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now >= expires_at
        } else {
            true
        }
    }
    
    /// 获取剩余有效时间（秒）
    pub fn remaining_time(&self) -> Option<u64> {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            if now < expires_at {
                Some(expires_at - now)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}

/// 构建完整URL（公共方法）
pub fn build_url(packet: &HttpPacket) -> String {
    let host = if !packet.host.is_empty() {
        packet.host.clone()
    } else {
        format!("{}:{}", packet.dst_ip, packet.dst_port)
    };
    
    let protocol = if packet.dst_port == 443 { "https" } else { "http" };
    let default_path = "/".to_string();
    let path = packet.path.as_ref().unwrap_or(&default_path);
    format!("{}://{}{}", protocol, host, path)
}

// 重新导出系统注册相关功能
pub use registry::{create_all_systems};