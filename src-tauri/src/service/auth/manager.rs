use crate::service::capture::HttpPacket;
use crate::service::auth::{
    store::{TokenStatus, TokenStore},
    systems::{self, SystemAuth},
    TokenEvent, send_token_event,
};
use anyhow::{Result, anyhow};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use log::{info, debug, error};
use std::time::{SystemTime, UNIX_EPOCH};


/// 简化的认证服务（移除内部事件系统）
pub struct AuthService {
    /// Token存储
    store: Arc<TokenStore>,
    /// 系统注册表
    systems: Arc<Mutex<HashMap<String, Box<dyn SystemAuth + Send + Sync>>>>,
}

impl AuthService {
    /// 创建新的认证服务（移除事件通道）
    pub async fn new() -> Self {
        let store = Arc::new(TokenStore::new());
        
        // 初始化系统
        let mut systems = HashMap::new();
        info!("🔧 开始加载所有认证系统...");
        
        for system in systems::create_all_systems() {
            let system_id = system.system_id().to_string();
            let system_name = system.system_name().to_string();
            info!("📦 加载系统: [{}] {}", system_id, system_name);
            systems.insert(system_id, system);
        }
        
        info!("🎯 所有认证系统加载完成，共 {} 个系统", systems.len());
        
        Self {
            store,
            systems: Arc::new(Mutex::new(systems)),
        }
    }
    
    /// 处理HTTP数据包
    pub async fn process_http_packet(&self, packet: HttpPacket) -> Result<()> {
        debug!("🔄 处理HTTP请求: {} {}", 
               packet.method.as_ref().unwrap_or(&"UNKNOWN".to_string()),
               packet.path.as_ref().unwrap_or(&"/".to_string()));
        
        // 只处理请求类型的数据包
        if packet.packet_type != "request" {
            return Ok(());
        }
        
        let url = format!("{}://{}{}", 
                         if packet.dst_port == 443 { "https" } else { "http" },
                         packet.host, 
                         packet.path.as_ref().unwrap_or(&"/".to_string()));
        
        let mut systems = self.systems.lock().await;
        let mut processed_count = 0;
        
        for (system_id, system) in systems.iter_mut() {
            debug!("🔍 系统 [{}] 开始检查请求", system_id);
            
            match system.process_http_request(&packet) {
                Ok(Some(token_info)) => {
                    processed_count += 1;
                    debug!("✅ 系统 [{}] 获取到新token", system_id);
                    
                    // 更新token存储
                    self.store.update_token(system_id.clone(), token_info.clone());
                    
                    // 发送token获取成功事件
                    if let Some(token) = &token_info.token {
                        let event = TokenEvent::TokenAcquired {
                            system_id: system_id.clone(),
                            system_name: system.system_name().to_string(),
                            token: token.clone(),
                            acquired_at: token_info.acquired_at.unwrap_or(0),
                            expires_at: token_info.expires_at.unwrap_or(0),
                            source_url: url.clone(),
                        };
                        
                        // 直接发送事件到前端（不经过内部通道）
                        send_token_event(event);
                        info!("📤 系统 [{}] 发送token更新事件", system_id);
                    }
                }
                Ok(None) => {
                    debug!("⏭️ 系统 [{}] 没有token更新", system_id);
                }
                Err(e) => {
                    debug!("⚠️ 系统 [{}] 处理失败: {}", system_id, e);
                }
            }
        }
        
        debug!("📊 请求处理完成，处理系统数量: {}", processed_count);
        Ok(())
    }
    
    /// 获取所有系统的token状态
    pub async fn get_all_token_status(&self) -> Vec<TokenStatus> {
        let systems = self.systems.lock().await;
        let system_names: HashMap<String, String> = systems
            .iter()
            .map(|(id, system)| (id.clone(), system.system_name().to_string()))
            .collect();
        
        self.store.get_all_status_with_names(&system_names)
    }
    
    /// 获取特定系统的token
    pub fn get_system_token(&self, system_id: &str) -> Option<String> {
        self.store.get_token(system_id)
    }
    
    /// 清除特定系统的token
    pub async fn clear_system_token(&self, system_id: &str) -> Result<()> {
        let systems = self.systems.lock().await;
        if systems.contains_key(system_id) {
            self.store.clear_token(system_id);
            Ok(())
        } else {
            Err(anyhow!("未找到系统: {}", system_id))
        }
    }
    
    /// 清除所有系统的token
    pub fn clear_all_tokens(&self) {
        self.store.clear_all_tokens();
    }
    
    /// 检查过期的token
    pub async fn check_expired_tokens(&self) -> Result<()> {
        debug!("⏰ 执行定期token过期检查...");
        
        let expired_systems = self.store.check_expired_tokens();
        
        if !expired_systems.is_empty() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            let systems = self.systems.lock().await;
            
            for system_id in expired_systems {
                if let Some(system) = systems.get(&system_id) {
                    let event = TokenEvent::TokenExpired {
                        system_id: system_id.clone(),
                        system_name: system.system_name().to_string(),
                        expired_at: now,
                    };
                    
                    // 直接发送过期事件到前端
                    send_token_event(event);
                }
            }
        }
        
        Ok(())
    }
    
    /// 启动过期检查器
    pub fn start_expiry_checker(&self) {
        let service = AuthService {
            store: self.store.clone(),
            systems: self.systems.clone(),
        };
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = service.check_expired_tokens().await {
                    error!("❌ 检查过期token失败: {}", e);
                }
            }
        });
        
        info!("⏰ Token过期检查器已启动");
    }
}

