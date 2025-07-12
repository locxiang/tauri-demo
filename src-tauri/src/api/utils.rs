#[tauri::command]
pub fn open_devtools(window: tauri::WebviewWindow) {
    window.open_devtools();
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
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
    
    log::info!("尝试打开文件夹: {}", expanded_path);
    
    std::process::Command::new(command)
        .arg(&expanded_path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}