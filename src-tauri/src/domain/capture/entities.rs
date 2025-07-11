use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 网络设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDevice {
    /// 设备名称
    pub name: String,
    /// 设备描述
    pub description: String,
    /// 是否为回环设备
    pub is_loopback: bool,
    /// IP地址列表
    pub addresses: Vec<String>,
}

/// 抓包状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureStatus {
    /// 是否正在运行
    pub is_running: bool,
    /// 已捕获的数据包数量
    pub packets_captured: u64,
    /// 已捕获的字节数
    pub bytes_captured: u64,
    /// 开始时间
    pub start_time: Option<u64>,
    /// 设备名称
    pub device_name: Option<String>,
    /// 丢弃的数据包数量
    pub dropped_packets: u64,
}

impl Default for CaptureStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            packets_captured: 0,
            bytes_captured: 0,
            start_time: None,
            device_name: None,
            dropped_packets: 0,
        }
    }
}

/// HTTP数据包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPacket {
    /// 数据包ID
    pub id: u64,
    /// 时间戳
    pub timestamp: u64,
    /// 源IP地址
    pub src_ip: String,
    /// 源端口
    pub src_port: u16,
    /// 目标IP地址
    pub dst_ip: String,
    /// 目标端口
    pub dst_port: u16,
    /// 数据包类型 (request/response)
    pub packet_type: String,
    
    // HTTP请求字段
    /// HTTP方法
    pub method: Option<String>,
    /// 请求路径
    pub path: Option<String>,
    
    // HTTP响应字段
    /// 状态码
    pub status_code: Option<u16>,
    /// 状态文本
    pub status_text: Option<String>,
    
    // 通用字段
    /// HTTP版本
    pub version: String,
    /// Host头部
    pub host: String,
    /// Content-Type
    pub content_type: String,
    /// Content-Length
    pub content_length: Option<usize>,
    /// 请求/响应头部
    pub headers: Vec<(String, String)>,
    /// 请求/响应体
    pub body: String,
}

impl HttpPacket {
    /// 创建新的HTTP数据包
    pub fn new(id: u64) -> Self {
        Self {
            id,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            src_ip: String::new(),
            src_port: 0,
            dst_ip: String::new(),
            dst_port: 0,
            packet_type: String::new(),
            method: None,
            path: None,
            status_code: None,
            status_text: None,
            version: String::new(),
            host: String::new(),
            content_type: String::new(),
            content_length: None,
            headers: Vec::new(),
            body: String::new(),
        }
    }
    
    /// 获取URL
    pub fn get_url(&self) -> String {
        if !self.host.is_empty() {
            format!("http://{}{}", self.host, self.path.as_ref().unwrap_or(&"/".to_string()))
        } else {
            format!("http://{}:{}{}", 
                   self.dst_ip, 
                   self.dst_port, 
                   self.path.as_ref().unwrap_or(&"/".to_string()))
        }
    }
    
    /// 获取指定名称的头部值
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.iter()
            .find(|(header_name, _)| header_name.to_lowercase() == name.to_lowercase())
            .map(|(_, value)| value)
    }
    
    /// 是否为HTTPS
    pub fn is_https(&self) -> bool {
        self.dst_port == 443
    }
}