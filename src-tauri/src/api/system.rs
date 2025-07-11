use tauri::{State, WebviewWindow};
use crate::app::{AppState, AppSetup};

/// 获取应用配置
#[tauri::command]
pub async fn get_app_config(
    state: State<'_, AppState>
) -> Result<crate::app::AppConfig, String> {
    log::info!("📋 获取应用配置");
    Ok((*state.config).clone())
}

/// 更新应用配置
#[tauri::command]
pub async fn update_app_config(
    _config: crate::app::AppConfig,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("📝 更新应用配置");
    
    // 这里需要获取可变引用，在实际实现中需要使用Arc<RwLock<>>
    // state.update_config(config).await
    //     .map_err(|e| e.to_string())
    
    Ok(())
}

/// 获取权限安装指导
#[tauri::command]
pub async fn get_permission_guide() -> Result<String, String> {
    log::info!("📖 获取权限安装指导");
    Ok(AppSetup::get_permission_guide())
}

/// 检查系统权限
#[tauri::command]
pub async fn check_system_permissions() -> Result<bool, String> {
    log::info!("🔒 检查系统权限");
    
    // 简化的权限检查
    #[cfg(target_os = "macos")]
    {
        let chmodbpf_path = "/Library/LaunchDaemons/org.wireshark.ChmodBPF.plist";
        Ok(std::path::Path::new(chmodbpf_path).exists())
    }
    
    #[cfg(target_os = "windows")]
    {
        let npcap_path = "C:\\Windows\\System32\\Npcap";
        Ok(std::path::Path::new(npcap_path).exists())
    }
    
    #[cfg(target_os = "linux")]
    {
        Ok(nix::unistd::geteuid().is_root())
    }
}

/// 获取系统信息
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    log::info!("💻 获取系统信息");
    
    let info = serde_json::json!({
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": env!("CARGO_PKG_VERSION"),
        "build_date": std::env::var("VERGEN_BUILD_DATE").unwrap_or_else(|_| "unknown".to_string()),
    });
    
    Ok(info)
}

/// 打开开发者工具
#[tauri::command]
pub async fn open_devtools(window: WebviewWindow) -> Result<(), String> {
    log::info!("🔧 打开开发者工具");
    window.open_devtools();
    Ok(())
}

/// 打开文件夹
#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    log::info!("📁 打开文件夹: {}", path);
    
    #[cfg(target_os = "macos")]
    let command = "open";
    
    #[cfg(target_os = "windows")]
    let command = "explorer";
    
    #[cfg(target_os = "linux")]
    let command = "xdg-open";
    
    // 展开路径中的 ~ 符号
    let expanded_path = if path.starts_with("~/") {
        match dirs::home_dir() {
            Some(mut home) => {
                home.push(&path[2..]);
                home.to_string_lossy().into_owned()
            },
            None => return Err("无法获取用户主目录".to_string())
        }
    } else {
        path
    };
    
    std::process::Command::new(command)
        .arg(&expanded_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}