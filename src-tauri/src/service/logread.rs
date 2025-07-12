use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc as async_mpsc};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager, Emitter};
use chrono;
use log::{Record, Metadata};
use std::time::Duration;
use tokio::time;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
    pub module: Option<String>,
}

pub struct LogManager {
    // Channel to broadcast batches of logs to subscribers
    broadcast_sender: broadcast::Sender<Vec<LogEntry>>,
    // In-memory log storage
    logs: Arc<Mutex<Vec<LogEntry>>>,
    // 异步组件发送器
    log_sender: Option<Arc<async_mpsc::Sender<LogEntry>>>,
    // 总日志计数器
    total_log_count: Arc<AtomicU64>,
}

static LOG_MANAGER: OnceCell<LogManager> = OnceCell::new();
static ASYNC_INITIALIZED: AtomicBool = AtomicBool::new(false);

impl LogManager {
    // 同步初始化基础组件
    pub fn init() {
        let (broadcast_sender, _) = broadcast::channel(1000);
        let logs = Arc::new(Mutex::new(Vec::new()));
        let total_log_count = Arc::new(AtomicU64::new(0));
        
        let manager = LogManager {
            broadcast_sender,
            logs,
            log_sender: None,
            total_log_count,
        };
        
        if LOG_MANAGER.set(manager).is_ok() {
            // 设置基础的混合日志记录器（同时支持同步和异步）
            let logger = HybridLogger::new();
            if log::set_boxed_logger(Box::new(logger)).is_ok() {
                log::set_max_level(log::LevelFilter::Debug);
            }
            log::info!("📊 日志管理器基础组件初始化完成");
        }
    }

    // 异步初始化流处理组件
    pub async fn init_async() -> Result<(), String> {
        if ASYNC_INITIALIZED.load(Ordering::Relaxed) {
            return Ok(());
        }

        let (log_sender, mut log_receiver) = async_mpsc::channel::<LogEntry>(2000);
        let log_sender_arc = Arc::new(log_sender);
        
        // 更新全局管理器的发送器
        if let Some(manager) = LOG_MANAGER.get() {
            // 通过unsafe方式更新，因为我们确保只在初始化时调用一次
            let manager_ptr = manager as *const LogManager as *mut LogManager;
            unsafe {
                (*manager_ptr).log_sender = Some(log_sender_arc.clone());
            }
        }
        
        let manager = Self::get();
        let logs = manager.logs.clone();
        let broadcast_sender = manager.broadcast_sender.clone();
        
        // 启动异步日志处理任务
        tokio::spawn(async move {
            let mut batch = Vec::with_capacity(50); // 减小批次大小，提高响应速度
            let mut interval = time::interval(Duration::from_millis(100)); // 减少间隔时间

            loop {
                tokio::select! {
                    Some(entry) = log_receiver.recv() => {
                        // 先添加到内存存储
                        {
                            let mut logs_guard = logs.lock().unwrap();
                            logs_guard.push(entry.clone());
                            
                            let len = logs_guard.len();
                            if len > 2000 {
                                logs_guard.drain(0..(len - 1500));
                            }
                        }
                        
                        batch.push(entry);

                        // 立即发送小批次或在达到一定数量时发送
                        if batch.len() >= 5 { // 进一步减小批次大小
                            if !batch.is_empty() {
                                let _ = broadcast_sender.send(batch.drain(..).collect());
                                interval.reset();
                            }
                        }
                    },
                    _ = interval.tick() => {
                        // 定期发送积累的日志
                        if !batch.is_empty() {
                            let _ = broadcast_sender.send(batch.drain(..).collect());
                        }
                    },
                }
            }
        });

        ASYNC_INITIALIZED.store(true, Ordering::Relaxed);
        log::info!("🚀 日志流管理器异步组件初始化完成");
        Ok(())
    }

    pub fn get() -> &'static LogManager {
        LOG_MANAGER.get().expect("日志管理器未初始化")
    }

    pub fn get_logs(&self, limit: usize) -> Vec<LogEntry> {
        let logs_guard = self.logs.lock().unwrap();
        logs_guard.iter().rev().take(limit).cloned().collect::<Vec<_>>().into_iter().rev().collect()
    }

    pub fn clear(&self) {
        self.logs.lock().unwrap().clear();
        // 注意：不清空总计数器，因为它表示累计产生的日志总数
    }

    pub fn get_total_count(&self) -> u64 {
        self.total_log_count.load(Ordering::Relaxed)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Vec<LogEntry>> {
        self.broadcast_sender.subscribe()
    }
}

// 混合日志记录器 - 根据初始化状态选择同步或异步处理
struct HybridLogger;

impl HybridLogger {
    fn new() -> Self {
        Self
    }
}

impl log::Log for HybridLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let entry = LogEntry {
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
                level: record.level().to_string().to_lowercase(),
                message: record.args().to_string(),
                module: record.module_path().map(|s| s.to_string()),
            };
            
            if let Some(manager) = LOG_MANAGER.get() {
                // 首先增加总计数器
                manager.total_log_count.fetch_add(1, Ordering::Relaxed);
                
                // 如果异步组件已初始化且有发送器，发送到异步通道
                if ASYNC_INITIALIZED.load(Ordering::Relaxed) {
                    if let Some(sender) = &manager.log_sender {
                        if sender.try_send(entry.clone()).is_ok() {
                            return; // 异步发送成功，异步任务会处理内存存储
                        }
                    }
                }
                
                // 异步发送失败或未初始化，使用同步方式直接存储
                let mut logs_guard = manager.logs.lock().unwrap();
                logs_guard.push(entry);
                
                let len = logs_guard.len();
                if len > 2000 {
                    logs_guard.drain(0..(len - 1500));
                }
            }
        }
    }

    fn flush(&self) {}
}

// These functions are the implementation details, not the commands themselves.
// The `#[tauri::command]` attribute is correctly placed in the `api` module.

pub async fn get_recent_logs(limit: Option<usize>) -> Result<Vec<LogEntry>, String> {
    Ok(LogManager::get().get_logs(limit.unwrap_or(1000)))
}

pub async fn get_total_log_count() -> Result<u64, String> {
    Ok(LogManager::get().get_total_count())
}

pub async fn clear_logs() -> Result<(), String> {
    LogManager::get().clear();
    Ok(())
}

pub async fn subscribe_log_stream(app_handle: AppHandle) -> Result<(), String> {
    let mut receiver = LogManager::get().subscribe();
    
    tokio::spawn(async move {
        while let Ok(log_batch) = receiver.recv().await {
            if !log_batch.is_empty() {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if let Err(e) = window.emit("log-stream-batch", &log_batch) {
                        log::error!("Failed to emit log batch: {}", e);
                    }
                }
            }
        }
    });
    
    Ok(())
}

pub async fn add_test_log(level: String, message: String) -> Result<(), String> {
    // 添加测试日志之前先确保异步组件已初始化
    if !ASYNC_INITIALIZED.load(Ordering::Relaxed) {
        LogManager::init_async().await.map_err(|e| format!("初始化异步组件失败: {}", e))?;
    }
    
    match level.as_str() {
        "error" => log::error!("🧪 测试: {}", message),
        "warn" => log::warn!("🧪 测试: {}", message),
        "info" => log::info!("🧪 测试: {}", message),
        "debug" => log::debug!("🧪 测试: {}", message),
        _ => log::info!("🧪 测试: {}", message),
    }
    Ok(())
}

// 添加一个启动测试日志生成器的函数
pub async fn start_test_log_generator() -> Result<(), String> {
    // 确保异步组件已初始化
    if !ASYNC_INITIALIZED.load(Ordering::Relaxed) {
        LogManager::init_async().await.map_err(|e| format!("初始化异步组件失败: {}", e))?;
    }
    
    tokio::spawn(async move {
        let mut counter = 0;
        let levels = ["info", "warn", "error", "debug"];
        
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            counter += 1;
            
            let level = levels[counter % levels.len()];
            let message = format!("定期测试日志 #{} - 时间: {}", counter, chrono::Utc::now().format("%H:%M:%S"));
            
            match level {
                "error" => log::error!("🔥 {}", message),
                "warn" => log::warn!("⚠️ {}", message),
                "info" => log::info!("ℹ️ {}", message),
                "debug" => log::debug!("🐛 {}", message),
                _ => log::info!("📝 {}", message),
            }
        }
    });
    
    log::info!("🚀 测试日志生成器已启动");
    Ok(())
}
