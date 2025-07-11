use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::ipc::Channel;

// 重新导出类型以供API层使用
pub use crate::domain::auth::entities::{TokenStatus, TokenEvent, TokenState};

/// 认证服务的简单实现
pub struct AuthService {
    token_store: Arc<Mutex<HashMap<String, String>>>,
    token_status: Arc<Mutex<Vec<crate::domain::auth::entities::TokenStatus>>>,
    event_channel: Arc<Mutex<Option<Channel<crate::domain::auth::entities::TokenEvent>>>>,
}

impl AuthService {
    pub fn new() -> Self {
        let mut status_list = Vec::new();
        
        // 初始化一些模拟的系统状态
        status_list.push(crate::domain::auth::entities::TokenStatus {
            system_id: "system_bi".to_string(),
            system_name: "BI系统".to_string(),
            has_token: false,
            token_acquired_at: None,
            token_expires_at: None,
            status: crate::domain::auth::entities::TokenState::Waiting,
        });
        
        status_list.push(crate::domain::auth::entities::TokenStatus {
            system_id: "system_three".to_string(),
            system_name: "三级治理中心".to_string(),
            has_token: false,
            token_acquired_at: None,
            token_expires_at: None,
            status: crate::domain::auth::entities::TokenState::Waiting,
        });
        
        status_list.push(crate::domain::auth::entities::TokenStatus {
            system_id: "system_drs".to_string(),
            system_name: "DRS系统".to_string(),
            has_token: false,
            token_acquired_at: None,
            token_expires_at: None,
            status: crate::domain::auth::entities::TokenState::Waiting,
        });
        
        Self {
            token_store: Arc::new(Mutex::new(HashMap::new())),
            token_status: Arc::new(Mutex::new(status_list)),
            event_channel: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_all_token_status(&self) -> Vec<crate::domain::auth::entities::TokenStatus> {
        self.token_status.lock().unwrap().clone()
    }

    pub async fn get_system_token(&self, system_id: &str) -> Option<String> {
        self.token_store.lock().unwrap().get(system_id).cloned()
    }

    pub async fn clear_system_token(&self, system_id: &str) -> Result<(), String> {
        self.token_store.lock().unwrap().remove(system_id);
        
        // 更新状态
        if let Ok(mut statuses) = self.token_status.lock() {
            for status in statuses.iter_mut() {
                if status.system_id == system_id {
                    status.has_token = false;
                    status.token_acquired_at = None;
                    status.token_expires_at = None;
                    status.status = crate::domain::auth::entities::TokenState::Waiting;
                }
            }
        }
        
        log::info!("✅ 清除系统Token成功: {}", system_id);
        Ok(())
    }

    pub fn clear_all_tokens(&self) {
        self.token_store.lock().unwrap().clear();
        
        // 更新所有状态
        if let Ok(mut statuses) = self.token_status.lock() {
            for status in statuses.iter_mut() {
                status.has_token = false;
                status.token_acquired_at = None;
                status.token_expires_at = None;
                status.status = TokenState::Waiting;
            }
        }
        
        log::info!("✅ 清除所有Token成功");
    }

    pub fn set_token_event_channel(&self, channel: Channel<crate::domain::auth::entities::TokenEvent>) -> Result<(), String> {
        *self.event_channel.lock().unwrap() = Some(channel);
        log::info!("✅ 设置Token事件通道成功");
        Ok(())
    }
}

// 全局单例
static AUTH_SERVICE: std::sync::OnceLock<AuthService> = std::sync::OnceLock::new();

pub fn get_auth_service() -> Option<&'static AuthService> {
    Some(AUTH_SERVICE.get_or_init(|| AuthService::new()))
}

// 为了兼容API调用，提供全局函数
pub async fn get_all_token_status() -> Vec<crate::domain::auth::entities::TokenStatus> {
    if let Some(service) = get_auth_service() {
        service.get_all_token_status().await
    } else {
        Vec::new()
    }
}

pub async fn get_system_token(system_id: &str) -> Option<String> {
    if let Some(service) = get_auth_service() {
        service.get_system_token(system_id).await
    } else {
        None
    }
}

pub fn set_token_event_channel(channel: Channel<crate::domain::auth::entities::TokenEvent>) -> Result<(), String> {
    if let Some(service) = get_auth_service() {
        service.set_token_event_channel(channel)
    } else {
        Err("认证服务未初始化".to_string())
    }
}