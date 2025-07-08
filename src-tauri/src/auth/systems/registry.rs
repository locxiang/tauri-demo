use super::{SystemAuth, system_bi, system_norm, system_c, system_d};

/// 系统注册中心
/// 
/// 这个文件专门负责管理所有认证系统的注册。
/// 当需要添加新系统时，只需要在这里添加相应的注册代码。
pub struct SystemRegistry;

impl SystemRegistry {
    /// 创建所有系统的实例
    /// 
    /// 这是系统注册的核心方法，返回所有已注册的认证系统实例。
    /// 添加新系统时，在这里添加对应的 create_system() 调用。
    pub fn create_all_systems() -> Vec<Box<dyn SystemAuth + Send + Sync>> {
        vec![
            // 系统A: BI系统 
            Box::new(system_bi::create_system()),
            
            // 系统B: 指标系统 
            Box::new(system_norm::create_system()),
            
            // 系统C: 数据平台 
            Box::new(system_c::create_system()),
            
            // 系统D: 业务系统 
            Box::new(system_d::create_system()),
            
            // 添加新系统时，在此处添加：
            // Box::new(system_xxx::create_system()),
        ]
    }
    
}

/// 便捷函数：创建所有系统的实例
/// 
/// 这是对外暴露的主要接口，其他模块通过这个函数获取所有系统实例。
pub fn create_all_systems() -> Vec<Box<dyn SystemAuth + Send + Sync>> {
    SystemRegistry::create_all_systems()
} 