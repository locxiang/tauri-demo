use crate::app::{AppConfig, AppState, AppResult};
use log::info;

/// 应用程序设置和初始化
pub struct AppSetup;

impl AppSetup {
    /// 初始化应用程序
    pub async fn initialize() -> AppResult<AppState> {
        info!("🚀 开始初始化应用程序...");
        
        // 1. 加载配置
        let config = AppConfig::load();
        info!("📋 配置加载完成");
        
        // 2. 初始化日志系统
        Self::setup_logging(&config)?;
        info!("📝 日志系统初始化完成");
        
        // 3. 检查系统权限
        Self::check_system_permissions(&config)?;
        info!("🔒 系统权限检查完成");
        
        // 4. 创建应用状态
        let app_state = AppState::new(config).await?;
        info!("📊 应用状态创建完成");
        
        info!("✅ 应用程序初始化完成");
        Ok(app_state)
    }
    
    /// 设置日志系统
    fn setup_logging(_config: &AppConfig) -> AppResult<()> {
        // 这里将在后续实现具体的日志配置
        // 目前保持现有的日志配置
        Ok(())
    }
    
    /// 检查系统权限
    fn check_system_permissions(_config: &AppConfig) -> AppResult<()> {
        // 检查网络抓包权限
        #[cfg(target_os = "macos")]
        {
            let chmodbpf_path = "/Library/LaunchDaemons/org.wireshark.ChmodBPF.plist";
            if std::path::Path::new(chmodbpf_path).exists() {
                info!("✅ macOS: ChmodBPF权限已配置");
            } else {
                log::warn!("⚠️ macOS: ChmodBPF权限未配置，抓包功能可能受限");
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            let npcap_path = "C:\\Windows\\System32\\Npcap";
            if std::path::Path::new(npcap_path).exists() {
                info!("✅ Windows: Npcap权限已配置");
            } else {
                log::warn!("⚠️ Windows: Npcap权限未配置，抓包功能可能受限");
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // 检查是否有root权限或CAP_NET_RAW权限
            if nix::unistd::geteuid().is_root() {
                info!("✅ Linux: 具有root权限");
            } else {
                log::warn!("⚠️ Linux: 缺少root权限，抓包功能可能受限");
            }
        }
        
        Ok(())
    }
    
    /// 获取权限安装指导
    pub fn get_permission_guide() -> String {
        #[cfg(target_os = "macos")]
        {
            "macOS权限配置指南：\n\
             1. 安装Wireshark: brew install --cask wireshark\n\
             2. 或手动安装ChmodBPF包\n\
             3. 重启应用程序".to_string()
        }
        
        #[cfg(target_os = "windows")]
        {
            "Windows权限配置指南：\n\
             1. 从 https://npcap.com/ 下载Npcap\n\
             2. 以管理员身份运行安装程序\n\
             3. 勾选WinPcap兼容模式\n\
             4. 重启应用程序".to_string()
        }
        
        #[cfg(target_os = "linux")]
        {
            "Linux权限配置指南：\n\
             1. 以root身份运行: sudo ./app\n\
             2. 或添加CAP_NET_RAW权限\n\
             3. 或将用户添加到pcap组".to_string()
        }
    }
}