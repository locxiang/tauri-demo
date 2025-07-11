use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("网络错误: {0}")]
    Network(#[from] NetworkError),
    
    #[error("认证错误: {0}")]
    Auth(#[from] AuthError),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Tauri错误: {0}")]
    Tauri(#[from] tauri::Error),
    
    #[error("pcap错误: {0}")]
    Pcap(#[from] pcap::Error),
    
    #[error("解析错误: {0}")]
    Parse(String),
    
    #[error("系统错误: {0}")]
    System(String),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("设备不存在: {0}")]
    DeviceNotFound(String),
    
    #[error("权限不足")]
    PermissionDenied,
    
    #[error("抓包失败: {0}")]
    CaptureError(String),
    
    #[error("协议解析失败: {0}")]
    ProtocolParseError(String),
    
    #[error("网络设备初始化失败")]
    DeviceInitError,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token无效")]
    InvalidToken,
    
    #[error("系统未注册: {0}")]
    SystemNotRegistered(String),
    
    #[error("Token已过期")]
    TokenExpired,
    
    #[error("认证服务未初始化")]
    ServiceNotInitialized,
    
    #[error("Token存储失败")]
    TokenStorageError,
}

pub type AppResult<T> = Result<T, AppError>;