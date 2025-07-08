use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 长度Token验证器
#[derive(Debug)]
struct LengthValidator;

impl TokenValidator for LengthValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 开始验证BI系统Cookie");
        
        // 检查是否包含x_login_pk字段
        if !token.contains("x_login_pk=") {
            let error_msg = "Cookie中缺少x_login_pk字段";
            warn!("❌ Cookie验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        // 提取x_login_pk的值
        let re = regex::Regex::new(r"x_login_pk=([^;]+)").unwrap();
        if let Some(captures) = re.captures(token) {
            if let Some(pk_value) = captures.get(1) {
                let value = pk_value.as_str();
                if value.is_empty() {
                    warn!("❌ Cookie验证失败: x_login_pk值为空");
                    return Err(anyhow!("x_login_pk值为空"));
                }
                debug!("✅ 找到有效的x_login_pk值");
            } else {
                warn!("❌ Cookie验证失败: 无法提取x_login_pk的值");
                return Err(anyhow!("无法提取x_login_pk的值"));
            }
        } else {
            warn!("❌ Cookie验证失败: x_login_pk格式不正确");
            return Err(anyhow!("x_login_pk格式不正确"));
        }
        
        info!("🔐 BI系统Cookie验证通过");
        Ok(())
    }
}

/// 创建GitLab系统实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_bi".to_string(),
        system_name: "BI系统".to_string(),
        url_pattern: r"https?://23\.210\.227\.16(:80)?/.*".to_string(),
        header_name: "Cookie".to_string(),
        token_pattern: r"(.+)".to_string(), // 直接匹配任意字符
        expires_duration: 3600, // 1小时
        validator: Box::new(LengthValidator),
    };
    
    BaseSystem::new(config)
} 