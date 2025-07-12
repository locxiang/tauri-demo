pub mod registry;
pub mod system_bi;
pub mod system_three;
pub mod system_drs;
pub mod system_test;

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::service::capture::HttpPacket;
use log::{info, warn, debug};
use regex::Regex;

/// 系统认证接口
pub trait SystemAuth {
    /// 获取系统ID
    fn system_id(&self) -> &str;
    
    /// 获取系统名称
    fn system_name(&self) -> &str;
    
    /// 处理HTTP数据包，尝试提取token（核心方法）
    /// 返回 Ok(Some(token_info)) 表示获取到新token
    /// 返回 Ok(None) 表示处理成功但没有token更新
    /// 返回 Err(e) 表示处理失败
    fn process_http_request(&mut self, packet: &HttpPacket) -> Result<Option<TokenInfo>>;
    
    /// 处理获取到的token
    fn handle_token(&mut self, token: &str, acquired_at: u64, expires_at: u64) -> Result<()>;
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

impl SystemConfig {

    /// 检查URL是否匹配
    fn matches_url(&self, url: &str) -> bool {
        debug!("🔍 系统[{}]检查URL匹配: {}", self.system_id, url);
        
        if let Ok(regex) = Regex::new(&self.url_pattern) {
            let matches = regex.is_match(url);
            matches
        } else {
            warn!("❌ 系统[{}] URL匹配正则表达式编译失败: {}", 
                  self.system_id, self.url_pattern);
            false
        }
    }

    /// 从HTTP数据包中提取token
    fn extract_token_from_request(&self, packet: &HttpPacket) -> Option<String> {
        debug!("🔎 系统[{}]开始提取token，Headers数量: {}", 
               self.system_id, packet.headers.len());
        
        // 查找指定的header
        let auth_header = packet.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(&self.header_name))
            .map(|(_, value)| value);
            
        let auth_header = match auth_header {
            Some(header) => {
                debug!("📋 系统[{}]找到{}header: {}...", 
                       self.system_id, self.header_name,
                       &header[..header.len().min(20)]);
                header
            }
            None => {
                debug!("❌ 系统[{}]未找到{}header", 
                       self.system_id, self.header_name);
                return None;
            }
        };

        // 使用正则提取token
        if let Ok(regex) = Regex::new(&self.token_pattern) {
            if let Some(captures) = regex.captures(auth_header) {
                if let Some(token_match) = captures.get(1) {
                    let token = token_match.as_str().to_string();
                    debug!("✅ 系统[{}]成功提取token，长度: {}", 
                           self.system_id, token.len());
                    return Some(token);
                }
            }
        }

        debug!("❌ 系统[{}]token提取失败，header值不匹配模式: {}", 
               self.system_id, self.token_pattern);
        None
    }
}

impl SystemAuth for SystemConfig {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn system_name(&self) -> &str {
        &self.system_name
    }
    
    fn process_http_request(&mut self, packet: &HttpPacket) -> Result<Option<TokenInfo>> {
        // 只处理HTTP请求，跳过响应
        if packet.packet_type != "request" {
            debug!("⏭️ 系统[{}]跳过HTTP{}处理", self.system_id, packet.packet_type);
            return Ok(None); // 没有token更新
        }
        
        let url = build_url(packet);
        
        // 检查URL是否匹配
        if !self.matches_url(&url) {
            debug!("⏭️ 系统[{}]跳过处理：URL不匹配", self.system_id);
            return Ok(None); // 没有token更新
        }
        
        info!("🎯 系统[{}]检测到匹配的URL: {}", self.system_id, url);
        
        // 提取token
        let token = match self.extract_token_from_request(packet) {
            Some(token) => {
                debug!("📨 系统[{}]成功提取到token", self.system_id);
                token
            }
            None => {
                debug!("📭 系统[{}]未找到有效的{}token", 
                       self.system_id, self.header_name);
                return Ok(None); // 没有token更新
            }
        };
        
        // 验证token
        if let Err(e) = self.validator.validate(&token) {
            warn!("❌ 系统[{}]token验证失败: {}", self.system_id, e);
            return Ok(None); // 没有token更新
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let expires_at = now + self.expires_duration;
        debug!("⏰ 系统[{}]更新token，过期时间: {} ({}秒后)", 
               self.system_id, expires_at, self.expires_duration);
        
        if let Err(e) = self.handle_token(&token, now, expires_at) {
            warn!("❌ 系统[{}]处理token失败: {}", self.system_id, e);
            return Ok(None); // 没有token更新
        }
        
        info!("🎉 系统[{}]token更新成功", self.system_id);
        
        // 创建新的TokenInfo返回
        let token_info = TokenInfo {
            token: Some(token),
            acquired_at: Some(now),
            expires_at: Some(expires_at),
            is_valid: true,
        };
        
        Ok(Some(token_info))
    }
    
    fn handle_token(&mut self, token: &str, acquired_at: u64, expires_at: u64) -> Result<()> {
        info!("🎯 系统[{}]处理新token，长度: {}，有效期: {}秒", 
              self.system_id, token.len(), expires_at - acquired_at);
        
        info!("✅ 系统[{}]token更新成功，过期时间: {}", 
              self.system_id, expires_at);
        Ok(())
    }
}

/// Token信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenInfo {
    pub token: Option<String>,
    pub acquired_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub is_valid: bool,
}

impl TokenInfo {
    
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
}

/// 构建完整URL（公共方法）
fn build_url(packet: &HttpPacket) -> String {
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