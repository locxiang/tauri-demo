use std::collections::HashMap;
use std::sync::{Arc, RwLock, atomic::{AtomicU64, Ordering}};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;
use chrono::Utc;
use once_cell::sync::OnceCell;
use tauri::Emitter;

/// 日志级别
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let level_str = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug", 
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        serializer.serialize_str(level_str)
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(serde::de::Error::custom(format!("Invalid log level: {}", s))),
        }
    }
}

impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Trace => LogLevel::Trace,
            log::Level::Debug => LogLevel::Debug,
            log::Level::Info => LogLevel::Info,
            log::Level::Warn => LogLevel::Warn,
            log::Level::Error => LogLevel::Error,
        }
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// 唯一ID
    pub id: u64,
    /// 时间戳（毫秒）
    pub timestamp: i64,
    /// 日志级别
    pub level: LogLevel,
    /// 目标模块
    pub target: String,
    /// 日志消息
    pub message: String,
    /// 上下文信息
    pub context: HashMap<String, serde_json::Value>,
    /// 文件名
    pub file: Option<String>,
    /// 行号
    pub line: Option<u32>,
}

/// 环形缓冲区
#[derive(Debug)]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    size: usize,
    pub capacity: usize,
}

impl<T> CircularBuffer<T> 
where 
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            size: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, item: T) {
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % self.capacity;
        
        if self.size < self.capacity {
            self.size += 1;
        }
    }
    
    pub fn get_recent(&self, limit: usize) -> Vec<T> {
        let mut result = Vec::new();
        let actual_limit = limit.min(self.size);
        
        for i in 0..actual_limit {
            let index = if self.head >= i + 1 {
                self.head - i - 1
            } else {
                self.capacity - (i + 1 - self.head)
            };
            
            if let Some(ref item) = self.buffer[index] {
                result.push(item.clone());
            }
        }
        
        result
    }
    
    pub fn len(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// 日志过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilters {
    pub level: Option<LogLevel>,
    pub keywords: Option<Vec<String>>,
    pub targets: Option<Vec<String>>,
    pub since: Option<i64>,
    pub until: Option<i64>,
}

impl LogFilters {
    pub fn matches(&self, entry: &LogEntry) -> bool {
        // 级别过滤
        if let Some(ref level) = self.level {
            if entry.level != *level {
                return false;
            }
        }
        
        // 关键词过滤
        if let Some(ref keywords) = self.keywords {
            if !keywords.is_empty() {
                let message_lower = entry.message.to_lowercase();
                let found = keywords.iter().any(|keyword| {
                    message_lower.contains(&keyword.to_lowercase())
                });
                if !found {
                    return false;
                }
            }
        }
        
        // 目标模块过滤
        if let Some(ref targets) = self.targets {
            if !targets.is_empty() && !targets.contains(&entry.target) {
                return false;
            }
        }
        
        // 时间范围过滤
        if let Some(since) = self.since {
            if entry.timestamp < since {
                return false;
            }
        }
        
        if let Some(until) = self.until {
            if entry.timestamp > until {
                return false;
            }
        }
        
        true
    }
}

/// 日志订阅者
#[derive(Debug)]
pub struct LogSubscriber {
    pub window: tauri::Window,
    pub filters: LogFilters,
    pub active: bool,
}

/// 现代化日志系统
pub struct ModernLogSystem {
    /// 环形缓冲区
    pub buffer: Arc<RwLock<CircularBuffer<LogEntry>>>,
    /// 广播发送器
    broadcaster: broadcast::Sender<LogEntry>,
    /// ID生成器
    id_counter: AtomicU64,
    /// 订阅者列表
    subscribers: Arc<RwLock<Vec<LogSubscriber>>>,
}

impl ModernLogSystem {
    pub fn new(buffer_capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(1000);
        
        Self {
            buffer: Arc::new(RwLock::new(CircularBuffer::new(buffer_capacity))),
            broadcaster: sender,
            id_counter: AtomicU64::new(1),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 添加日志条目
    pub fn add_log(&self, entry: LogEntry) {
        // 存储到环形缓冲区
        {
            let mut buffer = self.buffer.write().unwrap();
            buffer.push(entry.clone());
        }
        
        // 广播给订阅者
        let _ = self.broadcaster.send(entry.clone());
        
        // 推送给前端订阅者
        self.notify_subscribers(&entry);
    }
    
    /// 获取最近的日志
    pub fn get_recent_logs(&self, limit: usize, filters: Option<LogFilters>) -> Vec<LogEntry> {
        let buffer = self.buffer.read().unwrap();
        let mut logs = buffer.get_recent(limit);
        
        // 应用过滤器
        if let Some(filters) = filters {
            logs.retain(|entry| filters.matches(entry));
        }
        
        logs
    }
    
    /// 订阅日志流
    pub fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.broadcaster.subscribe()
    }
    
    /// 添加前端订阅者
    pub fn add_subscriber(&self, window: tauri::Window, filters: LogFilters) {
        let subscriber = LogSubscriber {
            window: window.clone(),
            filters,
            active: true,
        };
        
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.push(subscriber);
    }
    
    /// 移除订阅者
    pub fn remove_subscriber(&self, window_label: &str) {
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.retain(|sub| sub.window.label() != window_label);
    }
    
    /// 通知前端订阅者
    fn notify_subscribers(&self, entry: &LogEntry) {
        let subscribers = self.subscribers.read().unwrap();
        
        for subscriber in subscribers.iter() {
            if subscriber.active && subscriber.filters.matches(entry) {
                let _ = subscriber.window.emit("log-stream", entry);
            }
        }
    }
    
    /// 生成下一个ID
    fn next_id(&self) -> u64 {
        self.id_counter.fetch_add(1, Ordering::SeqCst)
    }
}

/// 自定义日志收集器
pub struct MemoryLogCollector {
    log_system: Arc<ModernLogSystem>,
}

impl MemoryLogCollector {
    pub fn new(log_system: Arc<ModernLogSystem>) -> Self {
        Self { log_system }
    }
}

impl log::Log for MemoryLogCollector {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug  // 改为Debug级别，允许更多日志
    }
    
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let entry = LogEntry {
                id: self.log_system.next_id(),
                timestamp: Utc::now().timestamp_millis(),
                level: record.level().into(),
                target: record.target().to_string(),
                message: record.args().to_string(),
                context: extract_context(record),
                file: record.file().map(|f| f.to_string()),
                line: record.line(),
            };
            
            self.log_system.add_log(entry);
        }
    }
    
    fn flush(&self) {
        // 内存日志无需刷新
    }
}

/// 提取上下文信息
fn extract_context(record: &log::Record) -> HashMap<String, serde_json::Value> {
    let mut context = HashMap::new();
    
    // 添加模块路径
    if let Some(module_path) = record.module_path() {
        context.insert("module_path".to_string(), serde_json::Value::String(module_path.to_string()));
    }
    
    // 可以在这里添加更多上下文信息提取逻辑
    
    context
}

/// 全局日志系统实例
static GLOBAL_LOG_SYSTEM: OnceCell<Arc<ModernLogSystem>> = OnceCell::new();

/// 初始化日志系统
pub fn init_log_system() -> Result<(), Box<dyn std::error::Error>> {
    let log_system = Arc::new(ModernLogSystem::new(10000)); // 保持最近1万条日志
    
    // 设置全局实例
    GLOBAL_LOG_SYSTEM.set(log_system.clone())
        .map_err(|_| "日志系统已初始化")?;
    
    // 设置自定义日志收集器
    let collector = MemoryLogCollector::new(log_system);
    log::set_boxed_logger(Box::new(collector))?;
    log::set_max_level(log::LevelFilter::Debug);  // 改为Debug级别
    
    Ok(())
}

/// 获取全局日志系统
pub fn get_log_system() -> Option<&'static Arc<ModernLogSystem>> {
    GLOBAL_LOG_SYSTEM.get()
}