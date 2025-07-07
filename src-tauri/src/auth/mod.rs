pub mod config;
pub mod systems;
pub mod events;
pub mod manager;

use anyhow::Result;
use crate::capture::HttpPacket;
use log::{info, debug, error};

// 重新导出主要类型
pub use config::*;
pub use events::*;

// 主要的初始化函数
pub fn init_auth_system() -> Result<()> {
    info!("🚀 开始初始化Token认证系统...");
    
    // 初始化事件系统
    debug!("📡 初始化事件系统...");
    events::init_event_system()?;
    debug!("✅ 事件系统初始化完成");
    
    // 初始化token管理器
    debug!("🎮 初始化token管理器...");
    manager::init_token_manager()?;
    debug!("✅ token管理器初始化完成");
    
    // 启动后台任务
    debug!("⏰ 启动token过期检查器...");
    manager::start_token_expiry_checker();
    debug!("✅ token过期检查器启动完成");
    
    info!("🔐 Token认证系统初始化完成！已加载 {} 个系统", 
          get_all_token_status().len());
    Ok(())
}

// 处理来自抓包模块的HTTP数据包
pub fn process_http_packet(packet: &HttpPacket) -> Result<()> {
    info!("🎯 auth模块收到HTTP{}: {} {} (来源: {}:{})", 
           if packet.packet_type == "request" { "请求" } else { "响应" },
           packet.method.as_ref().unwrap_or(&"UNKNOWN".to_string()),
           packet.path.as_ref().unwrap_or(&"/".to_string()),
           packet.src_ip, packet.src_port);
    
    info!("🔍 开始调用manager::process_incoming_request...");
    let result = manager::process_incoming_request(packet);
    
    match &result {
        Ok(_) => {
            info!("✅ auth模块处理HTTP{}成功", if packet.packet_type == "request" { "请求" } else { "响应" });
        }
        Err(e) => {
            error!("❌ auth模块处理HTTP{}失败: {}", if packet.packet_type == "request" { "请求" } else { "响应" }, e);
        }
    }
    
    result
}

// 获取所有系统的token状态
pub fn get_all_token_status() -> Vec<TokenStatus> {
    debug!("📊 获取所有系统token状态");
    let statuses = manager::get_all_token_status();
    
    debug!("📈 系统状态统计: 总数={}, 有效={}, 过期={}, 等待={}",
           statuses.len(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Active)).count(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Expired)).count(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Waiting)).count());
    
    statuses
}

// 获取特定系统的token
pub fn get_system_token(system_id: &str) -> Option<String> {
    debug!("🔍 查询系统 [{}] 的token", system_id);
    let token = manager::get_system_token(system_id);
    
    if token.is_some() {
        debug!("✅ 系统 [{}] token可用", system_id);
    } else {
        debug!("❌ 系统 [{}] token不可用", system_id);
    }
    
    token
} 