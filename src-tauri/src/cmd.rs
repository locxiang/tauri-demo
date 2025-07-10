use crate::capture;
use crate::auth;
use tauri::ipc::Channel;

// 获取捕获状态
#[tauri::command]
pub fn get_capture_status() -> capture::CaptureStatus {
    // 打印日志显示当前捕获状态
    println!("获取捕获状态: 正在返回当前状态");
    let status = capture::get_capture_status();
    println!("捕获状态: {:?}", status);
    status
}

// 设置状态更新通道
#[tauri::command]
pub fn set_status_channel(channel: Channel<capture::CaptureStatus>) -> Result<(), String> {
    capture::set_status_channel(channel).map_err(|e| e.to_string())
}

// 设置 HTTP 数据包通道
#[tauri::command]
pub fn set_http_channel(channel: Channel<capture::HttpPacket>) -> Result<(), String> {
    capture::set_http_channel(channel).map_err(|e| e.to_string())
}

// 启动数据包捕获
#[tauri::command]
pub fn init_capture(device_name: String) -> Result<(), String> {
    println!("start_capture: {}", device_name);
    capture::start_capture_with_device(device_name.clone()).map_err(|e| e.to_string())
}

// 停止数据包捕获
#[tauri::command]
pub fn stop_capture() -> Result<(), String> {
    capture::stop_capture().map_err(|e| e.to_string())
}

// 检查是否安装了ChmodBPF
#[tauri::command]
pub fn has_pcap() -> bool {
    capture::has_capture_prerequisites()
}

// 获取网络设备列表
#[tauri::command]
pub fn get_network_devices() -> Result<Vec<capture::NetworkDevice>, String> {
    capture::get_network_devices().map_err(|e| e.to_string())
}

// 创建数据包详情窗口
#[tauri::command]
pub async fn create_packet_window(
    app: tauri::AppHandle,
    title: String,
    url: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    println!("创建数据包详情窗口: {}", url);
    // 创建新窗口
    tauri::WebviewWindowBuilder::new(
        &app,
        &url,
        tauri::WebviewUrl::App(url.clone().into())
    )
    .title(&title)
    .inner_size(width, height)
    .center()
    .resizable(false)
    .minimizable(true)
    .maximizable(false)
    // .always_on_top(true) // 设置窗口始终在最前面
    .build()
    .map_err(|e| e.to_string())?;
        
    Ok(())
}

// 聚焦数据包详情窗口
#[tauri::command]
pub async fn focus_packet_window(
    _app: tauri::AppHandle,
    packet_id: String,
) -> Result<(), String> {
    let window_label = format!("packet-detail-{}", packet_id);
    println!("聚焦数据包详情窗口: {}", window_label);
    // 简化版本：总是返回成功
    Ok(())
}

// === Auth系统相关命令 ===

// 获取所有系统的token状态
#[tauri::command]
pub async fn get_all_token_status() -> Vec<auth::TokenStatus> {
    auth::get_all_token_status().await
}

// 获取特定系统的token
#[tauri::command]
pub async fn get_system_token(system_id: String) -> Option<String> {
    auth::get_system_token(&system_id).await
}

// 清除特定系统的token
#[tauri::command]
pub async fn clear_system_token(system_id: String) -> Result<(), String> {
    if let Some(service) = auth::get_auth_service() {
        service.clear_system_token(&system_id).await.map_err(|e| e.to_string())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

// 清除所有系统的token
#[tauri::command]
pub fn clear_all_tokens() -> Result<(), String> {
    if let Some(service) = auth::get_auth_service() {
        service.clear_all_tokens();
        Ok(())
    } else {
        Err("认证服务未初始化".to_string())
    }
}

// 设置Token事件通道
#[tauri::command]
pub fn set_token_event_channel(channel: Channel<auth::TokenEvent>) -> Result<(), String> {
    auth::set_token_event_channel_sync(channel).map_err(|e| e.to_string())
}


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