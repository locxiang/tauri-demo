use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use log::{info, debug};
use crate::auth::store::{TokenStore, TokenStatus};
use crate::auth::systems::{self, SystemAuth};
use crate::auth::TokenEvent;
use crate::auth::send_token_event;
use crate::capture::HttpPacket;





/// 简化的认证服务（移除内部事件系统）
pub struct AuthService {
    /// Token存储
    store: Arc<TokenStore>,
    /// 系统注册表
    systems: Arc<Mutex<HashMap<String, Box<dyn SystemAuth + Send + Sync>>>>,
    /// 存储每个系统最后一次命中 token 的 HTTP 请求包
    http_packets: Arc<Mutex<HashMap<String, HttpPacket>>>,
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
            http_packets: Arc::new(Mutex::new(HashMap::new())),
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
                    
                    // 存储命中 token 的 HTTP 包
                    let mut http_packets = self.http_packets.lock().await;
                    http_packets.insert(system_id.clone(), packet.clone());
                    debug!("📦 存储系统 [{}] 的命中请求包", system_id);
                    
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
        }
        Ok(())
    }

    /// 清除所有系统的token
    pub fn clear_all_tokens(&self) {
        self.store.clear_all_tokens();
    }
    
    
}

