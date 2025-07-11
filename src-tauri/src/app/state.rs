use std::sync::Arc;
use crate::app::config::AppConfig;

/// 应用全局状态
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    // 服务将在后续阶段添加
    // pub capture_service: Arc<CaptureService>,
    // pub auth_service: Arc<AuthService>,
}

impl AppState {
    /// 创建新的应用状态
    pub async fn new(config: AppConfig) -> crate::app::AppResult<Self> {
        let config = Arc::new(config);
        
        // 在后续阶段将初始化服务
        // let capture_service = Arc::new(CaptureService::new(config.clone()).await?);
        // let auth_service = Arc::new(AuthService::new(config.clone()).await?);
        
        Ok(Self {
            config,
            // capture_service,
            // auth_service,
        })
    }
    
    /// 获取配置
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
    
    /// 更新配置
    pub async fn update_config(&mut self, new_config: AppConfig) -> crate::app::AppResult<()> {
        // 保存配置到文件
        new_config.save()?;
        
        // 更新内存中的配置
        self.config = Arc::new(new_config);
        
        // 在后续阶段将通知服务配置已更新
        // self.capture_service.update_config(self.config.clone()).await?;
        // self.auth_service.update_config(self.config.clone()).await?;
        
        log::info!("应用配置已更新");
        Ok(())
    }
}