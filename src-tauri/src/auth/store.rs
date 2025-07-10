use crate::auth::{
    systems::TokenInfo,
};
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use log::{info, debug};

/// 无锁Token存储
pub struct TokenStore {
    /// 使用DashMap实现无锁并发访问
    tokens: DashMap<String, Arc<TokenInfo>>,
    /// 版本号，用于追踪更新
    version: AtomicU64,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: DashMap::new(),
            version: AtomicU64::new(0),
        }
    }
    
    /// 更新token
    pub fn update_token(&self, system_id: String, token_info: TokenInfo) {
        debug!("🔄 更新系统 [{}] 的token", system_id);
        self.tokens.insert(system_id.clone(), Arc::new(token_info));
        self.version.fetch_add(1, Ordering::Relaxed);
        info!("✅ 系统 [{}] token已更新", system_id);
    }
    
    /// 获取token
    pub fn get_token(&self, system_id: &str) -> Option<String> {
        self.tokens.get(system_id)
            .and_then(|entry| entry.value().token.clone())
    }
    
    /// 获取token信息
    pub fn get_token_info(&self, system_id: &str) -> Option<Arc<TokenInfo>> {
        self.tokens.get(system_id)
            .map(|entry| entry.value().clone())
    }
    
    /// 清除token
    pub fn clear_token(&self, system_id: &str) {
        if self.tokens.remove(system_id).is_some() {
            self.version.fetch_add(1, Ordering::Relaxed);
            info!("🗑️ 已清除系统 [{}] 的token", system_id);
        }
    }
    
    /// 清除所有token
    pub fn clear_all_tokens(&self) {
        let count = self.tokens.len();
        self.tokens.clear();
        if count > 0 {
            self.version.fetch_add(1, Ordering::Relaxed);
            info!("🗑️ 已清除所有token，共 {} 个", count);
        }
    }
    
    /// 获取所有系统的token状态（使用HashMap）
    pub fn get_all_status_with_names(&self, system_names: &std::collections::HashMap<String, String>) -> Vec<TokenStatus> {
        let mut statuses = Vec::new();
        
        for (system_id, system_name) in system_names.iter() {
            let status = if let Some(token_info) = self.get_token_info(system_id) {
                self.convert_token_info_to_status(&token_info, system_id, system_name)
            } else {
                TokenStatus {
                    system_id: system_id.clone(),
                    system_name: system_name.clone(),
                    has_token: false,
                    token_acquired_at: None,
                    token_expires_at: None,
                    last_seen_url: None,
                    status: crate::auth::store::TokenState::Waiting,
                }
            };
            
            statuses.push(status);
        }
        
        statuses
    }
 
    /// 将TokenInfo转换为TokenStatus
    fn convert_token_info_to_status(&self, info: &TokenInfo, system_id: &str, system_name: &str) -> TokenStatus {
        use crate::auth::store::TokenState;
        
        let status = if !info.is_valid {
            TokenState::Waiting
        } else if info.is_expired() {
            TokenState::Expired
        } else {
            TokenState::Active
        };
        
        TokenStatus {
            system_id: system_id.to_string(),
            system_name: system_name.to_string(),
            has_token: info.token.is_some(),
            token_acquired_at: info.acquired_at,
            token_expires_at: info.expires_at,
            last_seen_url: None,
            status,
        }
    }
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};

/// Token状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    pub system_id: String,
    pub system_name: String,
    pub has_token: bool,
    pub token_acquired_at: Option<u64>,
    pub token_expires_at: Option<u64>,
    pub last_seen_url: Option<String>,
    pub status: TokenState,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenState {
    /// 等待获取
    Waiting,
    /// 已获取，有效
    Active,
    /// 已过期
    Expired,
    /// 获取失败
    Failed(String),
} 