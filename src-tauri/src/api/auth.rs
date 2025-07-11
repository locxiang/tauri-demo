use tauri::{State, ipc::Channel};
use crate::app::AppState;
use crate::infrastructure::storage::{TokenEvent, TokenStatus};

/// 获取所有系统的Token状态
#[tauri::command]
pub async fn get_all_token_status(
    _state: State<'_, AppState>
) -> Result<Vec<TokenStatus>, String> {
    log::info!("📋 获取所有Token状态");
    
    // 调用认证模块的实现
    let statuses = crate::infrastructure::storage::get_all_token_status().await;
    log::info!("📊 返回 {} 个系统状态", statuses.len());
    
    Ok(statuses)
}

/// 获取指定系统的Token
#[tauri::command]
pub async fn get_system_token(
    system_id: String,
    _state: State<'_, AppState>
) -> Result<Option<String>, String> {
    log::info!("🔑 获取系统Token: {}", system_id);
    
    // 调用认证模块的实现
    let token = crate::infrastructure::storage::get_system_token(&system_id).await;
    
    Ok(token)
}

/// 清除指定系统的Token
#[tauri::command]
pub async fn clear_system_token(
    system_id: String,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🗑️ 清除系统Token: {}", system_id);
    
    // 调用认证模块的实现
    if let Some(auth_service) = crate::infrastructure::storage::get_auth_service() {
        auth_service.clear_system_token(&system_id).await
            .map_err(|e| e.to_string())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

/// 清除所有Token
#[tauri::command]
pub async fn clear_all_tokens(
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🗑️ 清除所有Token");
    
    // 调用认证模块的实现
    if let Some(auth_service) = crate::infrastructure::storage::get_auth_service() {
        auth_service.clear_all_tokens();
        Ok(())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

/// 设置Token事件通道
#[tauri::command]
pub async fn set_token_event_channel(
    channel: Channel<TokenEvent>,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🔗 设置Token事件通道");
    
    // 调用实际的Channel设置
    crate::infrastructure::storage::set_token_event_channel(channel)
        .map_err(|e| e.to_string())
}

/// 刷新Token状态
#[tauri::command]
pub async fn refresh_token_status(
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🔄 刷新Token状态");
    
    // 刷新功能实际上就是重新获取状态，前端会调用 get_all_token_status
    // 这里我们可以返回成功，具体的刷新逻辑由前端处理
    Ok(())
}