use crate::service::capture;
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