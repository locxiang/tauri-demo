use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Token状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenState {
    /// 活跃状态
    Active,
    /// 已过期
    Expired,
    /// 等待中
    Waiting,
    /// 失败
    Failed,
}

/// Token信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Token值
    pub token: Option<String>,
    /// 获取时间
    pub acquired_at: Option<u64>,
    /// 过期时间
    pub expires_at: Option<u64>,
    /// 是否有效
    pub is_valid: bool,
}

impl TokenInfo {
    /// 创建新的Token信息
    pub fn new(token: String, expires_in: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            token: Some(token),
            acquired_at: Some(now),
            expires_at: Some(now + expires_in),
            is_valid: true,
        }
    }
    
    /// 检查Token是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expires_at
        } else {
            true
        }
    }
    
    /// 获取剩余时间（秒）
    pub fn remaining_time(&self) -> Option<u64> {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if expires_at > now {
                Some(expires_at - now)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}

/// Token状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    /// 系统ID
    pub system_id: String,
    /// 系统名称
    pub system_name: String,
    /// 是否有Token
    pub has_token: bool,
    /// Token获取时间
    pub token_acquired_at: Option<u64>,
    /// Token过期时间
    pub token_expires_at: Option<u64>,
    /// 状态
    pub status: TokenState,
}

impl TokenStatus {
    /// 创建新的Token状态
    pub fn new(system_id: String, system_name: String) -> Self {
        Self {
            system_id,
            system_name,
            has_token: false,
            token_acquired_at: None,
            token_expires_at: None,
            status: TokenState::Waiting,
        }
    }
    
    /// 从TokenInfo更新状态
    pub fn update_from_token_info(&mut self, token_info: &TokenInfo) {
        self.has_token = token_info.token.is_some();
        self.token_acquired_at = token_info.acquired_at;
        self.token_expires_at = token_info.expires_at;
        
        if token_info.is_expired() {
            self.status = TokenState::Expired;
        } else if token_info.is_valid {
            self.status = TokenState::Active;
        } else {
            self.status = TokenState::Failed;
        }
    }
    
    /// 获取剩余时间（秒）
    pub fn remaining_time(&self) -> Option<u64> {
        if let Some(expires_at) = self.token_expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if expires_at > now {
                Some(expires_at - now)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}

/// 业务系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSystem {
    /// 系统ID
    pub system_id: String,
    /// 系统名称
    pub system_name: String,
    /// 系统URL模式
    pub url_pattern: String,
    /// 认证类型
    pub auth_type: AuthType,
    /// 是否启用
    pub enabled: bool,
}

/// 认证类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    /// Cookie认证
    Cookie {
        /// Cookie名称
        cookie_name: String,
    },
    /// Header认证
    Header {
        /// Header名称
        header_name: String,
    },
    /// 自定义认证
    Custom {
        /// 自定义提取逻辑
        extractor: String,
    },
}

/// Token事件
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl TokenEvent {
    /// 获取事件时间戳
    pub fn timestamp(&self) -> u64 {
        match self {
            TokenEvent::TokenAcquired { acquired_at, .. } => *acquired_at,
            TokenEvent::TokenExpired { expired_at, .. } => *expired_at,
            TokenEvent::TokenFailed { failed_at, .. } => *failed_at,
        }
    }
    
    /// 获取系统ID
    pub fn system_id(&self) -> &str {
        match self {
            TokenEvent::TokenAcquired { system_id, .. } => system_id,
            TokenEvent::TokenExpired { system_id, .. } => system_id,
            TokenEvent::TokenFailed { system_id, .. } => system_id,
        }
    }
    
    /// 获取系统名称
    pub fn system_name(&self) -> &str {
        match self {
            TokenEvent::TokenAcquired { system_name, .. } => system_name,
            TokenEvent::TokenExpired { system_name, .. } => system_name,
            TokenEvent::TokenFailed { system_name, .. } => system_name,
        }
    }
}