use tauri::ipc::Channel;
use crate::service::auth;

// 获取所有系统的token状态
#[tauri::command]
pub async fn get_all_token_status() -> Vec<auth::TokenStatus> {
    auth::get_all_token_status().await
}

// 获取特定系统的token
#[tauri::command]
pub async fn get_system_token(system_id: String) -> Option<String> {
    auth::get_system_token(&system_id).await
}

// 清除特定系统的token
#[tauri::command]
pub async fn clear_system_token(system_id: String) -> Result<(), String> {
    if let Some(service) = auth::get_auth_service() {
        service.clear_system_token(&system_id).await.map_err(|e| e.to_string())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

// 清除所有系统的token
#[tauri::command]
pub fn clear_all_tokens() -> Result<(), String> {
    if let Some(service) = auth::get_auth_service() {
        service.clear_all_tokens();
        Ok(())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

// 设置Token事件通道
#[tauri::command]
pub fn set_token_event_channel(channel: Channel<auth::TokenEvent>) -> Result<(), String> {
    auth::set_token_event_channel_sync(channel).map_err(|e| e.to_string())
}