use anyhow::{anyhow, Result};
use etherparse::{NetSlice, SlicedPacket, TransportSlice};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use pcap::Capture;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::ipc::Channel;

// 运行状态控制
static CAPTURE_RUNNING: OnceCell<Arc<AtomicBool>> = OnceCell::new();
static CAPTURE_THREAD: OnceCell<Arc<Mutex<Option<thread::JoinHandle<()>>>>> = OnceCell::new();
static CAPTURE_STATUS: OnceCell<Arc<Mutex<CaptureStatus>>> = OnceCell::new();
static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();
static STATUS_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<CaptureStatus>>>>> = OnceCell::new();
static HTTP_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<HttpPacket>>>>> = OnceCell::new();

// 捕获状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureStatus {
    pub running: bool,
    pub message: String,
    pub device_name: String,
    pub start_time: u64,
}

// HTTP 数据包结构（统一处理请求和响应）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPacket {
    pub id: u64,
    pub timestamp: u64,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    pub packet_type: String, // "request" 或 "response"
    
    // 请求字段
    pub method: Option<String>,
    pub path: Option<String>,
    
    // 响应字段
    pub status_code: Option<u16>,
    pub status_text: Option<String>,
    
    // 通用字段
    pub version: String,
    pub host: String,
    pub content_type: String,
    pub content_length: Option<usize>,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

// 网络设备结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub name: String,
    pub description: String,
    pub is_loopback: bool,
    pub addresses: Vec<String>,
}

// 初始化 AppHandle 以便发送事件
pub fn init_app_handle(app_handle: tauri::AppHandle) -> Result<()> {
    APP_HANDLE
        .set(app_handle)
        .map_err(|_| anyhow!("已经初始化过 AppHandle"))?;
    Ok(())
}

// 设置状态通道
pub fn set_status_channel(channel: Channel<CaptureStatus>) -> Result<()> {
    if let Some(channels) = STATUS_CHANNEL.get() {
        let mut guard = channels.lock().unwrap();
        *guard = Some(channel);
        Ok(())
    } else {
        let channels = Arc::new(Mutex::new(Some(channel)));
        STATUS_CHANNEL
            .set(channels)
            .map_err(|_| anyhow!("已经初始化过状态通道"))?;
        Ok(())
    }
}

// 设置 HTTP 数据包通道
pub fn set_http_channel(channel: Channel<HttpPacket>) -> Result<()> {
    if let Some(channels) = HTTP_CHANNEL.get() {
        let mut guard = channels.lock().unwrap();
        *guard = Some(channel);
        Ok(())
    } else {
        let channels = Arc::new(Mutex::new(Some(channel)));
        HTTP_CHANNEL
            .set(channels)
            .map_err(|_| anyhow!("已经初始化过 HTTP 数据包通道"))?;
        Ok(())
    }
}

pub fn init_capture(device_name: String) -> Result<()> {
    info!("init_capture");
    info!("准备在设备 {:?} 上初始化数据包捕获", device_name.clone());
    // 检查设备名称是否为空
    if device_name.trim().is_empty() {
        return Err(anyhow!("未指定网络设备名称，无法启动捕获"));
    }
    
    
    // 如果已经在运行，先停止
    if let Some(status) = CAPTURE_STATUS.get() {
        let status_guard = status.lock().unwrap();
        if status_guard.running {
            drop(status_guard); // 释放锁
            info!("检测到捕获正在运行，先停止...");
            stop_capture()?;
            // 等待一段时间确保完全停止
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    // 初始化或获取运行状态标志
    let running = if let Some(existing_running) = CAPTURE_RUNNING.get() {
        // 重置现有的运行标志
        existing_running.store(true, Ordering::Relaxed);
        existing_running.clone()
    } else {
        // 首次初始化
        let running = Arc::new(AtomicBool::new(true));
        CAPTURE_RUNNING
            .set(running.clone())
            .map_err(|_| anyhow!("已经初始化过运行状态标志"))?;
        running
    };

    // 初始化或获取线程句柄
    let thread_handle = if let Some(existing_handle) = CAPTURE_THREAD.get() {
        existing_handle.clone()
    } else {
        let thread_handle = Arc::new(Mutex::new(None));
        CAPTURE_THREAD
            .set(thread_handle.clone())
            .map_err(|_| anyhow!("已经初始化过线程句柄"))?;
        thread_handle
    };
        
    // 初始化或更新捕获状态
    let status = if let Some(existing_status) = CAPTURE_STATUS.get() {
        let mut status_guard = existing_status.lock().unwrap();
        status_guard.running = true;
        status_guard.message = "正在初始化...".to_string();
        status_guard.device_name = "未知".to_string();
        status_guard.start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        existing_status.clone()
    } else {
        let status = Arc::new(Mutex::new(CaptureStatus {
            running: true,
            message: "正在初始化...".to_string(),
            device_name: "".to_string(),
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }));
        CAPTURE_STATUS
            .set(status.clone())
            .map_err(|_| anyhow!("已经初始化过捕获状态"))?;
        status
    };
        
    // 初始化通道存储（如果尚未初始化）
    if STATUS_CHANNEL.get().is_none() {
        STATUS_CHANNEL
            .set(Arc::new(Mutex::new(None)))
            .map_err(|_| anyhow!("已经初始化过状态通道存储"))?;
    }
    
    if HTTP_CHANNEL.get().is_none() {
        HTTP_CHANNEL
            .set(Arc::new(Mutex::new(None)))
            .map_err(|_| anyhow!("已经初始化过 HTTP 数据包通道存储"))?;
    }

    // 清理旧的线程句柄（如果存在）
    {
        let mut handle_guard = thread_handle.lock().unwrap();
        if let Some(old_thread) = handle_guard.take() {
            if old_thread.is_finished() {
                let _ = old_thread.join();
                info!("清理了旧的捕获线程");
            }
        }
    }

    // 启动捕获线程
    let running_clone = running.clone();
    let status_clone = status.clone();
    let capture_thread = thread::spawn(move || {
        if let Err(e) = start_capture(running_clone, status_clone, device_name.clone()) {
            error!("数据包捕获出错: {}", e);
            update_capture_status(Some(false), Some(format!("捕获失败: {}", e)), None);
        }
    });

    // 保存线程句柄
    *thread_handle.lock().unwrap() = Some(capture_thread);
    info!("数据包捕获线程已启动");
    
    // 发送初始状态更新
    send_status_update();
    Ok(())
}

fn start_capture(running: Arc<AtomicBool>, status: Arc<Mutex<CaptureStatus>>, device_name: String) -> Result<()> {
    info!("开始初始化数据包捕获...");
    
    // 更新状态
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.message = "正在初始化网络捕获...".to_string();
    }
    send_status_update();

    // 获取可用的网络设备列表
    let list = match pcap::Device::list() {
        Ok(list) => list,
        Err(e) => {
                    let err = anyhow!("获取网络设备列表失败: {}", e);
        update_capture_status(Some(false), Some(err.to_string()), None);
        return Err(err);
        }
    };
    
    if list.is_empty() {
        let err = anyhow!("没有找到可用的网络设备");
        update_capture_status(Some(false), Some(err.to_string()), None);
        return Err(err);
    }
    
    // 根据指定的设备名称查找设备
    let device = if !device_name.is_empty() {
        // 查找指定名称的设备
        match list.iter().find(|d| d.name == device_name) {
            Some(device) => {
                info!("找到指定的网络设备: {}", device_name);
                device
            },
            None => {
                let err = anyhow!("未找到指定的网络设备: {}", device_name);
                update_capture_status(Some(false), Some(err.to_string()), None);
                return Err(err);
            }
        }
    } else {
        // 如果没有指定设备名称，直接报错
        let err = anyhow!("未指定网络设备名称，请选择一个网络设备");
        update_capture_status(Some(false), Some(err.to_string()), None);
        return Err(err);
    };
    
    info!("使用网络设备: {}", device.name);
    
    // 更新状态
    update_capture_status(None, None, Some(device_name.clone()));
    
    let mut cap = match Capture::from_device(device.clone()) {
        Ok(cap) => match cap.promisc(true).timeout(1000).immediate_mode(true).open() {
            Ok(cap) => cap,
            Err(e) => {
                let err = anyhow!("打开网络设备失败: {}. 请确保已安装ChmodBPF", e);
                update_capture_status(Some(false), Some(err.to_string()), None);
                return Err(err);
            }
        },
        Err(e) => {
            let err = anyhow!("创建捕获句柄失败: {}. 请确保已安装ChmodBPF", e);
            update_capture_status(Some(false), Some(err.to_string()), None);
            return Err(err);
        }
    };

    // 设置过滤器，只捕获 HTTP 流量
    if let Err(e) = cap.filter("tcp port 80 or tcp port 8080 or tcp port 443", true) {
        let err = anyhow!("设置过滤器失败: {}", e);
        update_capture_status(Some(false), Some(err.to_string()), None);
        return Err(err);
    }
    
    // 更新状态为运行中
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.running = true;
        status_guard.message = "正在捕获 HTTP 请求和响应...".to_string();
    }
    send_status_update();
    
    info!("开始捕获 HTTP 请求和响应数据包...");

    // 简化的捕获循环
    while running.load(Ordering::Relaxed) {
        match cap.next_packet() {
            Ok(packet) => {
                debug!("捕获到数据包: {} 字节", packet.data.len());
                match SlicedPacket::from_ethernet(packet.data) {
                    Ok(sliced) => process_packet(sliced),
                    Err(e) => debug!("解析数据包错误: {:?}", e)
                }
            },
            Err(pcap::Error::TimeoutExpired) => continue, // 超时是正常的
            Err(e) => {
                error!("捕获数据包错误: {:?}", e);
                if !running.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    // 更新状态为已停止
    update_capture_status(Some(false), Some("数据包捕获已停止".to_string()), None);

    info!("数据包捕获已停止");
    Ok(())
}

fn process_packet(sliced: SlicedPacket) {
    // 提取 IP 地址信息
    let (src_ip, dst_ip) = match sliced.net {
        Some(NetSlice::Ipv4(ipv4)) => (
            IpAddr::V4(ipv4.header().source_addr()),
            IpAddr::V4(ipv4.header().destination_addr()),
        ),
        Some(NetSlice::Ipv6(ipv6)) => (
            IpAddr::V6(ipv6.header().source_addr()),
            IpAddr::V6(ipv6.header().destination_addr()),
        ),
        _ => return,
    };

    // 提取端口信息和payload
    let (src_port, dst_port, payload) = match sliced.transport {
        Some(TransportSlice::Tcp(tcp)) => (tcp.source_port(), tcp.destination_port(), tcp.payload()),
        Some(TransportSlice::Udp(udp)) => (udp.source_port(), udp.destination_port(), udp.payload()),
        _ => return,
    };

    // 只处理有效载荷
    if !payload.is_empty() {
        // 检查是否是 HTTP 数据包（请求或响应）
        if let Some(packet_type) = detect_http_packet_type(payload) {
            // 根据类型解析 HTTP 数据包
            let mut http_packet = match packet_type.as_str() {
                "request" => parse_http_request(payload),
                "response" => parse_http_response(payload),
                _ => None,
            };
            
            if let Some(ref mut packet) = http_packet {
                // 添加网络信息
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                packet.timestamp = timestamp;
                packet.src_ip = src_ip.to_string();
                packet.src_port = src_port;
                packet.dst_ip = dst_ip.to_string();
                packet.dst_port = dst_port;
                packet.packet_type = packet_type.clone();
                
                // 生成唯一ID
                packet.id = timestamp * 1000 + (src_port as u64 % 1000);
                
                // 输出格式化的 HTTP 信息到日志
                match packet_type.as_str() {
                    "request" => {
                        info!("捕获 HTTP 请求: {}:{} -> {}:{}", src_ip, src_port, dst_ip, dst_port);
                        if let Some(method) = &packet.method {
                            info!("请求方法: {}", method);
                        }
                        if let Some(path) = &packet.path {
                            info!("请求路径: {}", path);
                        }
                    }
                    "response" => {
                        info!("捕获 HTTP 响应: {}:{} -> {}:{}", src_ip, src_port, dst_ip, dst_port);
                        if let Some(status_code) = packet.status_code {
                            info!("响应状态码: {}", status_code);
                        }
                        if let Some(status_text) = &packet.status_text {
                            info!("响应状态: {}", status_text);
                        }
                    }
                    _ => {}
                }
                
                // 🔐 新增：将HTTP数据包发送给认证系统处理
                info!("📨 发送HTTP{}到认证系统处理...", if packet_type == "request" { "请求" } else { "响应" });
                if let Err(e) = crate::auth::process_http_packet(&packet) {
                    error!("❌ 认证系统处理HTTP{}失败: {}", if packet_type == "request" { "请求" } else { "响应" }, e);
                } else {
                    debug!("✅ 认证系统处理HTTP{}成功", if packet_type == "request" { "请求" } else { "响应" });
                }
                
                // 发送 HTTP 数据包到前端
                send_http_packet(packet.clone());
            }
        }
    }
}

// 检测 HTTP 数据包类型（请求或响应）
fn detect_http_packet_type(data: &[u8]) -> Option<String> {
    if data.len() < 4 {
        return None;
    }

    // 检查是否是 HTTP 请求
    if data.starts_with(b"GET ")
        || data.starts_with(b"POST ")
        || data.starts_with(b"PUT ")
        || data.starts_with(b"DELETE ")
        || data.starts_with(b"HEAD ")
        || data.starts_with(b"OPTIONS ")
        || data.starts_with(b"PATCH ")
        || data.starts_with(b"TRACE ")
        || data.starts_with(b"CONNECT ")
    {
        return Some("request".to_string());
    }

    // 检查是否是 HTTP 响应
    if data.starts_with(b"HTTP/1.0 ")
        || data.starts_with(b"HTTP/1.1 ")
        || data.starts_with(b"HTTP/2.0 ")
        || data.starts_with(b"HTTP/3.0 ")
    {
        return Some("response".to_string());
    }

    None
}

// 解析 HTTP 请求
fn parse_http_request(data: &[u8]) -> Option<HttpPacket> {
    let http_text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = http_text.split("\r\n").collect();
    
    if lines.is_empty() {
        return None;
    }
    
    // 解析请求行
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if request_line_parts.len() < 3 {
        return None;
    }
    
    let method = request_line_parts[0].to_string();
    let path = request_line_parts[1].to_string();
    let version = request_line_parts[2].to_string();
    
    let mut host = String::new();
    let mut content_type = String::new();
    let mut content_length = None;
    let mut headers = Vec::new();
    let mut body = String::new();
    
    // 找到请求头和请求体的分隔位置
    let mut body_start = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        
        // 解析请求头
        if i > 0 {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let header_name = parts[0].to_string();
                let header_value = parts[1].to_string();
                
                // 提取特定的头信息
                match header_name.to_lowercase().as_str() {
                    "host" => host = header_value.clone(),
                    "content-type" => content_type = header_value.clone(),
                    "content-length" => {
                        if let Ok(len) = header_value.parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                    _ => {}
                }
                
                headers.push((header_name, header_value));
            }
        }
    }
    
    // 提取请求体
    if body_start < lines.len() {
        body = lines[body_start..].join("\r\n");
    }
    
    Some(HttpPacket {
        id: 0, // 将在 process_packet 中设置
        timestamp: 0, // 将在 process_packet 中设置
        src_ip: String::new(), // 将在 process_packet 中设置
        src_port: 0, // 将在 process_packet 中设置
        dst_ip: String::new(), // 将在 process_packet 中设置
        dst_port: 0, // 将在 process_packet 中设置
        packet_type: "request".to_string(),
        method: Some(method),
        path: Some(path),
        status_code: None,
        status_text: None,
        version,
        host,
        content_type,
        content_length,
        headers,
        body,
    })
}

// 解析 HTTP 响应
fn parse_http_response(data: &[u8]) -> Option<HttpPacket> {
    let http_text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = http_text.split("\r\n").collect();
    
    if lines.is_empty() {
        return None;
    }
    
    // 解析状态行
    let status_line_parts: Vec<&str> = lines[0].splitn(3, ' ').collect();
    if status_line_parts.len() < 3 {
        return None;
    }
    
    let version = status_line_parts[0].to_string();
    let status_code = match status_line_parts[1].parse::<u16>() {
        Ok(code) => code,
        Err(_) => return None,
    };
    let status_text = status_line_parts[2].to_string();
    
    let host = String::new();
    let mut content_type = String::new();
    let mut content_length = None;
    let mut headers = Vec::new();
    let mut body = String::new();
    
    // 找到响应头和响应体的分隔位置
    let mut body_start = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        
        // 解析响应头
        if i > 0 {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let header_name = parts[0].to_string();
                let header_value = parts[1].to_string();
                
                // 提取特定的头信息
                match header_name.to_lowercase().as_str() {
                    "content-type" => content_type = header_value.clone(),
                    "content-length" => {
                        if let Ok(len) = header_value.parse::<usize>() {
                            content_length = Some(len);
                        }
                    }
                    _ => {}
                }
                
                headers.push((header_name, header_value));
            }
        }
    }
    
    // 提取响应体
    if body_start < lines.len() {
        body = lines[body_start..].join("\r\n");
    }
    
    Some(HttpPacket {
        id: 0, // 将在 process_packet 中设置
        timestamp: 0, // 将在 process_packet 中设置
        src_ip: String::new(), // 将在 process_packet 中设置
        src_port: 0, // 将在 process_packet 中设置
        dst_ip: String::new(), // 将在 process_packet 中设置
        dst_port: 0, // 将在 process_packet 中设置
        packet_type: "response".to_string(),
        method: None,
        path: None,
        status_code: Some(status_code),
        status_text: Some(status_text),
        version,
        host,
        content_type,
        content_length,
        headers,
        body,
    })
}

pub fn stop_capture() -> Result<()> {
    info!("正在停止数据包捕获...");
    
    // 设置运行标志为 false，通知捕获线程停止
    if let Some(running) = CAPTURE_RUNNING.get() {
        running.store(false, Ordering::Relaxed);
        info!("已发送停止数据包捕获的信号");
    }

    // 等待线程结束，给一个合理的超时时间
    if let Some(handle) = CAPTURE_THREAD.get() {
        let mut guard = handle.lock().unwrap();
        if let Some(thread) = guard.take() {
            // 释放锁，允许线程正常执行
            drop(guard);
            
            // 等待线程结束，最多等待3秒
            let mut attempts = 0;
            while !thread.is_finished() && attempts < 30 {
                std::thread::sleep(std::time::Duration::from_millis(100));
                attempts += 1;
            }
            
            if thread.is_finished() {
                let _ = thread.join();
                info!("数据包捕获线程已正常结束");
            } else {
                info!("等待线程结束超时，强制继续");
                // 注意：这里不调用 join，因为线程可能还在运行
            }
        }
    }
    
    // 更新状态
    update_capture_status(Some(false), Some("数据包捕获已停止".to_string()), None);
    
    info!("数据包捕获停止完成");
    Ok(())
}

// 获取捕获状态
pub fn get_capture_status() -> CaptureStatus {
    if let Some(status) = CAPTURE_STATUS.get() {
        let status_guard = status.lock().unwrap();
        status_guard.clone()
    } else {
        CaptureStatus {
            running: false,
            message: "捕获未初始化".to_string(),
            device_name: "".to_string(),
            start_time: 0,
        }
    }
}

// 通过 Channel 发送状态更新
fn send_status_update() {
    if let Some(channels) = STATUS_CHANNEL.get() {
        let guard = channels.lock().unwrap();
        if let Some(channel) = &*guard {
            let status = get_capture_status();
            info!("通过 Channel 发送状态更新: {:?}", status);
            if let Err(e) = channel.send(status) {
                error!("发送状态更新失败: {}", e);
            }
        }
    }
}

// 优雅的状态更新函数
fn update_capture_status(running: Option<bool>, message: Option<String>, device_name: Option<String>) {
    if let Some(status) = CAPTURE_STATUS.get() {
        let mut status_guard = status.lock().unwrap();
        
        if let Some(running_val) = running {
            status_guard.running = running_val;
        }
        
        if let Some(message_val) = message {
            status_guard.message = message_val;
        }
        
        if let Some(device_name_val) = device_name {
            status_guard.device_name = device_name_val;
        }
        
        // 如果停止运行，不更新开始时间；如果开始运行，更新开始时间
        if let Some(true) = running {
            status_guard.start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
        }
    }
    
    // 自动发送状态更新
    send_status_update();
}

// 通过 Channel 发送 HTTP 数据包
fn send_http_packet(packet: HttpPacket) {
    if let Some(channels) = HTTP_CHANNEL.get() {
        let guard = channels.lock().unwrap();
        if let Some(channel) = &*guard {
            info!("通过 Channel 发送 HTTP {}: {:?}", 
                if packet.packet_type == "request" { "请求" } else { "响应" }, 
                packet);
            if let Err(e) = channel.send(packet) {
                error!("发送 HTTP 数据包失败: {}", e);
            }
        }
    }
}

// 获取网络设备列表
pub fn get_network_devices() -> Result<Vec<NetworkDevice>> {
    let list = match pcap::Device::list() {
        Ok(list) => list,
        Err(e) => {
            return Err(anyhow!("获取网络设备列表失败: {}", e));
        }
    };

    let mut devices = Vec::new();
    for device in list {
        let description = device.desc.unwrap_or_else(|| "无描述".to_string());
        let is_loopback = device.flags.is_loopback();
        let addresses = device.addresses.iter()
            .map(|addr| addr.addr.to_string())
            .collect();

        let network_device = NetworkDevice {
            name: device.name.clone(),
            description,
            is_loopback,
            addresses,
        };

        devices.push(network_device);
    }

    info!("找到 {} 个网络设备", devices.len());
    Ok(devices)
}

/// 检查是否安装了抓包所需的系统组件（macOS上是ChmodBPF）
#[cfg(target_os = "macos")]
pub fn has_capture_prerequisites() -> bool {
    use log::{info};
    use std::path::Path;

    // 检查ChmodBPF服务是否存在
    info!("检查ChmodBPF服务是否已安装...");
    let chmodbpf_path = Path::new("/Library/LaunchDaemons/org.wireshark.ChmodBPF.plist");
    if !chmodbpf_path.exists() {
        return false;
    }

    // 打印ChmodBPF服务状态日志
    info!("ChmodBPF服务文件存在，检查服务状态...");
    
    
    return true;
        
}

/// 在Windows上检查是否安装了Npcap
#[cfg(target_os = "windows")]
pub fn has_capture_prerequisites() -> bool {
    use log::{info};
    use std::path::Path;
    
    // 检查Npcap是否已安装
    info!("检查Windows上Npcap是否已安装...");
    
    // 检查Npcap安装路径
    let npcap_path = Path::new("C:\\Windows\\System32\\Npcap");
    if npcap_path.exists() {
        info!("检测到Npcap安装目录存在");
        return true;
    }
    
    return false;
}

/// 在其他平台上，默认返回false
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn has_capture_prerequisites() -> bool {
    false;
}
