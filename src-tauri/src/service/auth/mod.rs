pub mod systems;
pub mod manager;
pub mod store;

use anyhow::Result;
use crate::service::capture::HttpPacket;
use log::{info, error};
use std::sync::{Arc, Mutex};
use manager::AuthService;
use once_cell::sync::OnceCell;
use tauri::ipc::Channel;

// 重新导出主要类型
pub use store::{TokenStatus};

// 全局静态变量存储Token事件通道（参考抓包模块的实现）
static TOKEN_EVENT_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<TokenEvent>>>>> = OnceCell::new();
static AUTH_SERVICE: OnceCell<Arc<AuthService>> = OnceCell::new();

/// Token事件类型 - 和前端保持完全一致
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum TokenEvent {
    /// Token获取成功
    TokenAcquired {
        system_id: String,
        system_name: String,
        token: String,
        acquired_at: u64,
        expires_at: u64,
        source_url: String,
    },
    /// Token过期
    TokenExpired {
        system_id: String,
        system_name: String,
        expired_at: u64,
    },
    /// Token获取失败
    TokenFailed {
        system_id: String,
        system_name: String,
        error: String,
        failed_at: u64,
    },
}

/// 设置Token事件通道（参考抓包模块）
pub fn set_token_event_channel(channel: Channel<TokenEvent>) -> Result<()> {
    if let Some(channels) = TOKEN_EVENT_CHANNEL.get() {
        let mut guard = channels.lock().unwrap();
        *guard = Some(channel);
        info!("🔗 Token事件通道已设置");
        Ok(())
    } else {
        let channels = Arc::new(Mutex::new(Some(channel)));
        TOKEN_EVENT_CHANNEL
            .set(channels)
            .map_err(|_| anyhow::anyhow!("已经初始化过Token事件通道"))?;
        info!("🔗 Token事件通道已初始化");
        Ok(())
    }
}

/// 直接发送Token事件到前端（参考抓包模块）
pub fn send_token_event(event: TokenEvent) {
    if let Some(channels) = TOKEN_EVENT_CHANNEL.get() {
        if let Ok(guard) = channels.try_lock() {
            if let Some(channel) = &*guard {
                info!("📤 发送Token事件到前端: {:?}", event);
                let channel_clone = channel.clone();
                drop(guard); // 立即释放锁
                if let Err(e) = channel_clone.send(event) {
                    error!("❌ 发送Token事件失败: {}", e);
                } else {
                    info!("✅ Token事件发送成功");
                }
            } else {
                error!("❌ Token事件通道未设置");
            }
        } else {
            error!("❌ Token事件通道正忙，跳过发送");
        }
    } else {
        error!("❌ Token事件通道未初始化");
    }
}

/// 初始化简化的认证系统
pub async fn init_auth_system() -> Result<()> {
    info!("🚀 开始初始化简化的Token认证系统...");
    
    // 初始化Token事件通道存储
    if TOKEN_EVENT_CHANNEL.get().is_none() {
        TOKEN_EVENT_CHANNEL
            .set(Arc::new(Mutex::new(None)))
            .map_err(|_| anyhow::anyhow!("Token事件通道存储已经初始化过"))?;
    }
    
    // 创建认证服务
    let auth_service = Arc::new(AuthService::new().await);
    
    // 设置全局认证服务
    AUTH_SERVICE
        .set(auth_service.clone())
        .map_err(|_| anyhow::anyhow!("认证服务已经初始化过"))?;
    
    // 启动过期检查器
    auth_service.start_expiry_checker();
    
    info!("🔐 简化的Token认证系统初始化完成！");
    Ok(())
}

/// 获取认证服务实例
pub fn get_auth_service() -> Option<Arc<AuthService>> {
    AUTH_SERVICE.get().cloned()
}

/// 处理来自抓包模块的HTTP数据包
pub async fn process_http_packet(packet: &HttpPacket) -> Result<()> {
    info!("🎯 auth模块收到HTTP{}: {} {} (来源: {}:{})", 
           if packet.packet_type == "request" { "请求" } else { "响应" },
           packet.method.as_ref().unwrap_or(&"UNKNOWN".to_string()),
           packet.path.as_ref().unwrap_or(&"/".to_string()),
           packet.src_ip, packet.src_port);
    
    if let Some(auth_service) = get_auth_service() {
        let result = auth_service.process_http_packet(packet.clone()).await;
        
        match &result {
            Ok(_) => {
                info!("✅ auth模块已处理HTTP{}", if packet.packet_type == "request" { "请求" } else { "响应" });
            }
            Err(e) => {
                error!("❌ auth模块处理HTTP{}失败: {}", if packet.packet_type == "request" { "请求" } else { "响应" }, e);
            }
        }
        
        result
    } else {
        error!("❌ 认证系统未初始化");
        Err(anyhow::anyhow!("认证系统未初始化"))
    }
}

/// 获取所有系统的token状态
pub async fn get_all_token_status() -> Vec<TokenStatus> {
    if let Some(auth_service) = get_auth_service() {
        auth_service.get_all_token_status().await
    } else {
        error!("❌ 认证系统未初始化，返回空状态");
        Vec::new()
    }
}

/// 获取特定系统的token
pub async fn get_system_token(system_id: &str) -> Option<String> {
    if let Some(auth_service) = get_auth_service() {
        auth_service.get_system_token(system_id)
    } else {
        error!("❌ 认证系统未初始化，无法获取token");
        None
    }
}

/// 设置前端Token事件通道
pub fn set_token_event_channel_sync(channel: Channel<TokenEvent>) -> Result<()> {
    set_token_event_channel(channel)
}