use super::{TokenValidator, SystemConfig};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 数据Token验证器
#[derive(Debug)]
struct DataTokenValidator;

impl TokenValidator for DataTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 数据Token验证器开始验证token，长度: {}", token.len());
        
        if token.len() <= 10 {
            let error_msg = format!("数据平台token长度必须大于10位，当前: {}", token.len());
            warn!("❌ 数据Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        info!("🔐 数据Token验证通过");
        Ok(())
    }
}

/// 创建系统C实例
pub fn create_system() -> SystemConfig {
    let config = SystemConfig {
        system_id: "system_three".to_string(),
        system_name: "三级治理中心门户".to_string(),
        url_pattern: r"http?://23\.210\.52\.91:8080/api/.*".to_string(),
        header_name: "Authorization".to_string(),
        token_pattern: r"(.+)".to_string(), // 直接匹配任意字符
        expires_duration: 1800, // 30分钟
        validator: Box::new(DataTokenValidator),
    };
    
    config
} 