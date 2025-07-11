use std::sync::{Arc, Mutex};
use tauri::ipc::Channel;

// 重新导出类型以供API层使用
pub use crate::domain::capture::entities::{NetworkDevice, CaptureStatus, HttpPacket};

/// 网络捕获服务的简单实现
pub struct NetworkCaptureService {
    status: Arc<Mutex<crate::domain::capture::entities::CaptureStatus>>,
    status_channel: Arc<Mutex<Option<Channel<crate::domain::capture::entities::CaptureStatus>>>>,
    http_channel: Arc<Mutex<Option<Channel<crate::domain::capture::entities::HttpPacket>>>>,
}

impl NetworkCaptureService {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(crate::domain::capture::entities::CaptureStatus::default())),
            status_channel: Arc::new(Mutex::new(None)),
            http_channel: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_capture_status(&self) -> crate::domain::capture::entities::CaptureStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn start_capture_with_device(&self, device_name: String) -> Result<(), String> {
        let mut status = self.status.lock().unwrap();
        status.is_running = true;
        status.device_name = Some(device_name);
        status.start_time = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        log::info!("✅ 模拟启动抓包成功");
        Ok(())
    }

    pub fn stop_capture(&self) -> Result<(), String> {
        let mut status = self.status.lock().unwrap();
        status.is_running = false;
        status.device_name = None;
        status.start_time = None;
        
        log::info!("✅ 模拟停止抓包成功");
        Ok(())
    }

    pub fn get_network_devices(&self) -> Result<Vec<crate::domain::capture::entities::NetworkDevice>, String> {
        // 返回模拟的网络设备列表
        Ok(vec![
            crate::domain::capture::entities::NetworkDevice {
                name: "lo0".to_string(),
                description: "Loopback".to_string(),
                is_loopback: true,
                addresses: vec!["127.0.0.1".to_string()],
            },
            crate::domain::capture::entities::NetworkDevice {
                name: "en0".to_string(),
                description: "Wi-Fi".to_string(),
                is_loopback: false,
                addresses: vec!["192.168.1.100".to_string()],
            },
        ])
    }

    pub fn set_status_channel(&self, channel: Channel<crate::domain::capture::entities::CaptureStatus>) -> Result<(), String> {
        *self.status_channel.lock().unwrap() = Some(channel);
        log::info!("✅ 设置抓包状态通道成功");
        Ok(())
    }

    pub fn set_http_channel(&self, channel: Channel<crate::domain::capture::entities::HttpPacket>) -> Result<(), String> {
        *self.http_channel.lock().unwrap() = Some(channel);
        log::info!("✅ 设置HTTP数据包通道成功");
        Ok(())
    }

    pub fn has_capture_prerequisites(&self) -> bool {
        // 简单的权限检查，实际环境中可以更复杂
        true
    }
}

// 全局单例
static CAPTURE_SERVICE: std::sync::OnceLock<NetworkCaptureService> = std::sync::OnceLock::new();

pub fn get_capture_service() -> &'static NetworkCaptureService {
    CAPTURE_SERVICE.get_or_init(|| NetworkCaptureService::new())
}

// 为了兼容API调用，提供全局函数
pub fn get_capture_status() -> crate::domain::capture::entities::CaptureStatus {
    get_capture_service().get_capture_status()
}

pub fn start_capture_with_device(device_name: String) -> Result<(), String> {
    get_capture_service().start_capture_with_device(device_name)
}

pub fn stop_capture() -> Result<(), String> {
    get_capture_service().stop_capture()
}

pub fn get_network_devices() -> Result<Vec<crate::domain::capture::entities::NetworkDevice>, String> {
    get_capture_service().get_network_devices()
}

pub fn set_status_channel(channel: Channel<crate::domain::capture::entities::CaptureStatus>) -> Result<(), String> {
    get_capture_service().set_status_channel(channel)
}

pub fn set_http_channel(channel: Channel<crate::domain::capture::entities::HttpPacket>) -> Result<(), String> {
    get_capture_service().set_http_channel(channel)
}

pub fn has_capture_prerequisites() -> bool {
    get_capture_service().has_capture_prerequisites()
}