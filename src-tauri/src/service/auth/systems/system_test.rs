use super::{TokenValidator, SystemConfig};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 长度Token验证器
#[derive(Debug)]
struct LengthValidator;

impl TokenValidator for LengthValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 开始验证测试系统Cookie");
        
        // 检查是否包含wdcid字段
        if !token.contains("wdcid=") {
            let error_msg = "Cookie中缺少wdcid字段";
            warn!("❌ Cookie验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        // 提取wdcid的值
        let re = regex::Regex::new(r"wdcid=([^;]+)").unwrap();
        if let Some(captures) = re.captures(token) {
            if let Some(pk_value) = captures.get(1) {
                let value = pk_value.as_str();
                if value.is_empty() {
                    warn!("❌ Cookie验证失败: wdcid值为空");
                    return Err(anyhow!("wdcid值为空"));
                }
                debug!("✅ 找到有效的wdcid值");
            } else {
                warn!("❌ Cookie验证失败: 无法提取wdcid的值");
                return Err(anyhow!("无法提取wdcid的值"));
            }
        } else {
            warn!("❌ Cookie验证失败: wdcid格式不正确");
            return Err(anyhow!("wdcid格式不正确"));
        }
        
        info!("🔐 测试系统Cookie验证通过");
        Ok(())
    }
}

/// 创建GitLab系统实例
pub fn create_system() -> SystemConfig {
    let config = SystemConfig {
        system_id: "system_test".to_string(),
        system_name: "测试系统（moe.gov.cn）".to_string(),
        url_pattern: r"https?://www\.moe\.gov\.cn/.*".to_string(),
        header_name: "Cookie".to_string(),
        token_pattern: r"(.+)".to_string(), // 直接匹配任意字符
        expires_duration: 3600, // 1小时
        validator: Box::new(LengthValidator),
    };
    
    config
} 
