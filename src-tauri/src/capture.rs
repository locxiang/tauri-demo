use pcap::Device;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub name: String,
    pub description: Option<String>,
    pub is_up: bool,
    pub is_running: bool,
    pub is_loopback: bool,
}

/// 获取网卡数量
#[tauri::command]
pub fn get_network_device_count() -> Result<usize, String> {
    match Device::list() {
        Ok(devices) => Ok(devices.len()),
        Err(e) => Err(format!("获取网络设备失败: {}", e)),
    }
}

/// 获取所有网络设备信息
#[tauri::command]
pub fn get_network_devices() -> Result<Vec<NetworkDevice>, String> {
    match Device::list() {
        Ok(devices) => {
            let network_devices: Vec<NetworkDevice> = devices
                .into_iter()
                .map(|device| NetworkDevice {
                    name: device.name,
                    description: device.desc,
                    is_up: device.flags.is_up(),
                    is_running: device.flags.is_running(),
                    is_loopback: device.flags.is_loopback(),
                })
                .collect();
            Ok(network_devices)
        }
        Err(e) => Err(format!("获取网络设备列表失败: {}", e)),
    }
}

/// 获取活跃网卡数量（排除回环设备）
#[tauri::command]
pub fn get_active_network_device_count() -> Result<usize, String> {
    match Device::list() {
        Ok(devices) => {
            let active_count = devices
                .into_iter()
                .filter(|device| {
                    device.flags.is_up() 
                    && device.flags.is_running() 
                    && !device.flags.is_loopback()
                })
                .count();
            Ok(active_count)
        }
        Err(e) => Err(format!("获取活跃网络设备失败: {}", e)),
    }
}
