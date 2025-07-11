use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    /// 抓包缓冲区大小
    pub capture_buffer_size: usize,
    /// 抓包超时时间（毫秒）
    pub capture_timeout: u64,
    /// 最大数据包大小
    pub max_packet_size: usize,
    /// BPF过滤表达式
    pub filter_expression: String,
    /// 是否启用混杂模式
    pub promiscuous_mode: bool,
    /// 最大数据包队列长度
    pub max_packet_queue_size: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    /// Token刷新间隔（秒）
    pub token_refresh_interval: u64,
    /// Token过期阈值（秒）
    pub token_expire_threshold: u64,
    /// 最大Token缓存大小
    pub max_token_cache_size: usize,
    /// 是否启用Token自动刷新
    pub auto_refresh_tokens: bool,
    /// Token事件历史最大数量
    pub max_token_event_history: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 最大日志文件大小（字节）
    pub max_file_size: u64,
    /// 最大日志文件数量
    pub max_files: usize,
    /// 是否启用控制台输出
    pub console_output: bool,
    /// 日志文件路径
    pub log_file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemConfig {
    /// 权限检查间隔（秒）
    pub permission_check_interval: u64,
    /// 是否启用性能监控
    pub enable_performance_monitoring: bool,
    /// 自动选择网络设备
    pub auto_select_network_device: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                capture_buffer_size: 1_000_000,
                capture_timeout: 1000,
                max_packet_size: 65535,
                filter_expression: "tcp port 80 or tcp port 8080 or tcp port 443".to_string(),
                promiscuous_mode: true,
                max_packet_queue_size: 10000,
            },
            auth: AuthConfig {
                token_refresh_interval: 60,
                token_expire_threshold: 300,
                max_token_cache_size: 100,
                auto_refresh_tokens: true,
                max_token_event_history: 1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                max_file_size: 10_000_000,
                max_files: 5,
                console_output: true,
                log_file_path: None,
            },
            system: SystemConfig {
                permission_check_interval: 300,
                enable_performance_monitoring: true,
                auto_select_network_device: true,
            },
        }
    }
}

impl AppConfig {
    /// 从文件加载配置
    pub fn from_file(path: &PathBuf) -> crate::app::AppResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)
            .map_err(|e| crate::app::AppError::Config(e.to_string()))?;
        Ok(config)
    }
    
    /// 保存配置到文件
    pub fn save_to_file(&self, path: &PathBuf) -> crate::app::AppResult<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::app::AppError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// 获取配置文件路径
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("big-data-rpa-v4");
            path.push("config.toml");
            path
        })
    }
    
    /// 加载配置（优先从文件加载，失败则使用默认配置）
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_path() {
            if config_path.exists() {
                match Self::from_file(&config_path) {
                    Ok(config) => {
                        log::info!("配置已从文件加载: {:?}", config_path);
                        return config;
                    }
                    Err(e) => {
                        log::warn!("加载配置文件失败: {}, 使用默认配置", e);
                    }
                }
            }
        }
        
        let default_config = Self::default();
        log::info!("使用默认配置");
        default_config
    }
    
    /// 保存当前配置
    pub fn save(&self) -> crate::app::AppResult<()> {
        if let Some(config_path) = Self::config_path() {
            // 确保目录存在
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            self.save_to_file(&config_path)?;
            log::info!("配置已保存到: {:?}", config_path);
        }
        Ok(())
    }
}