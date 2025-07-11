use tauri::{State, ipc::Channel};
use crate::app::AppState;
use crate::domain::capture::entities::NetworkDevice;
use crate::infrastructure::network::{CaptureStatus, HttpPacket};

/// 获取抓包状态
#[tauri::command]
pub async fn get_capture_status(
    _state: State<'_, AppState>
) -> Result<CaptureStatus, String> {
    log::info!("📊 获取抓包状态");
    Ok(crate::infrastructure::network::get_capture_status())
}

/// 初始化抓包
#[tauri::command]
pub async fn init_capture(
    device_name: String,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🚀 启动抓包: {}", device_name);
    
    // 调用实际的抓包实现
    crate::infrastructure::network::start_capture_with_device(device_name)
        .map_err(|e| e.to_string())
}

/// 停止抓包
#[tauri::command]
pub async fn stop_capture(
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("⏹️ 停止抓包");
    
    // 调用实际的抓包实现
    crate::infrastructure::network::stop_capture()
        .map_err(|e| e.to_string())
}

/// 获取网络设备列表
#[tauri::command]
pub async fn get_network_devices(
    _state: State<'_, AppState>
) -> Result<Vec<NetworkDevice>, String> {
    log::info!("📡 获取网络设备列表");
    
    // 调用实际的实现并转换类型
    let devices = crate::infrastructure::network::get_network_devices()
        .map_err(|e| e.to_string())?;
    
    // 转换为domain层的NetworkDevice类型
    let converted_devices = devices.into_iter().map(|device| {
        NetworkDevice {
            name: device.name,
            description: device.description,
            is_loopback: device.is_loopback,
            addresses: device.addresses,
        }
    }).collect();
    
    Ok(converted_devices)
}

/// 设置抓包状态通道
#[tauri::command]
pub async fn set_status_channel(
    channel: Channel<CaptureStatus>,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🔗 设置抓包状态通道");
    
    // 调用实际的Channel设置
    crate::infrastructure::network::set_status_channel(channel)
        .map_err(|e| e.to_string())
}

/// 设置HTTP数据包通道
#[tauri::command]
pub async fn set_http_channel(
    channel: Channel<HttpPacket>,
    _state: State<'_, AppState>
) -> Result<(), String> {
    log::info!("🔗 设置HTTP数据包通道");
    
    // 调用实际的Channel设置
    crate::infrastructure::network::set_http_channel(channel)
        .map_err(|e| e.to_string())
}

/// 检查是否有抓包权限
#[tauri::command]
pub async fn has_pcap(
    _state: State<'_, AppState>
) -> Result<bool, String> {
    log::info!("🔒 检查抓包权限");
    
    // 调用实际的权限检查
    Ok(crate::infrastructure::network::has_capture_prerequisites())
}