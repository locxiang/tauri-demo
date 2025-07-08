use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 业务Token验证器
#[derive(Debug)]
struct BusinessTokenValidator;

impl TokenValidator for BusinessTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 业务Token验证器开始验证token，长度: {}", token.len());
        
        // 检查是否包含pdp_cqdrs_session字段
        if !token.contains("pdp_cqdrs_session=") {
            let error_msg = "Cookie中缺少pdp_cqdrs_session字段";
            warn!("❌ 业务Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        // 提取pdp_cqdrs_session的值
        let re = regex::Regex::new(r"pdp_cqdrs_session=([^;]+)").unwrap();
        if let Some(captures) = re.captures(token) {
            if let Some(session_value) = captures.get(1) {
                let value = session_value.as_str();
                if value.is_empty() {
                    warn!("❌ 业务Token验证失败: pdp_cqdrs_session值为空");
                    return Err(anyhow!("pdp_cqdrs_session值为空"));
                }
                debug!("✅ 找到有效的pdp_cqdrs_session值");
            } else {
                warn!("❌ 业务Token验证失败: 无法提取pdp_cqdrs_session的值");
                return Err(anyhow!("无法提取pdp_cqdrs_session的值"));
            }
        } else {
            warn!("❌ 业务Token验证失败: pdp_cqdrs_session格式不正确");
            return Err(anyhow!("pdp_cqdrs_session格式不正确"));
        }
        
        info!("🔐 业务Token验证通过");
        Ok(())
    }
}

/// 创建系统D实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_drs".to_string(),
        system_name: "DRS 系统".to_string(),
        url_pattern: r"https?://23\.210\.52\.94(:80)?/.*".to_string(),
        header_name: "Cookie".to_string(),
        token_pattern: r"(.+)".to_string(), // 直接匹配任意字符
        expires_duration: 1200, // 20分钟
        validator: Box::new(BusinessTokenValidator),
    };
    
    BaseSystem::new(config)
} 