use tauri::{State, Window};
use crate::app::logging::{get_log_system, LogFilters, LogEntry};

/// 获取最近的日志
#[tauri::command]
pub async fn get_recent_logs(
    limit: Option<usize>,
    filters: Option<LogFilters>,
    _state: State<'_, crate::app::AppState>
) -> Result<Vec<LogEntry>, String> {
    log::info!("📋 获取最近日志，限制: {:?}", limit);
    
    if let Some(log_system) = get_log_system() {
        let logs = log_system.get_recent_logs(limit.unwrap_or(1000), filters);
        log::info!("📊 返回 {} 条日志", logs.len());
        Ok(logs)
    } else {
        Err("日志系统未初始化".to_string())
    }
}

/// 订阅日志流
#[tauri::command]
pub async fn subscribe_log_stream(
    window: Window,
    filters: Option<LogFilters>,
    _state: State<'_, crate::app::AppState>
) -> Result<(), String> {
    log::info!("🔗 窗口 {} 订阅日志流", window.label());
    
    if let Some(log_system) = get_log_system() {
        let filters = filters.unwrap_or(LogFilters {
            level: None,
            keywords: None,
            targets: None,
            since: None,
            until: None,
        });
        
        log_system.add_subscriber(window, filters);
        Ok(())
    } else {
        Err("日志系统未初始化".to_string())
    }
}

/// 取消订阅日志流
#[tauri::command]
pub async fn unsubscribe_log_stream(
    window: Window,
    _state: State<'_, crate::app::AppState>
) -> Result<(), String> {
    log::info!("❌ 窗口 {} 取消订阅日志流", window.label());
    
    if let Some(log_system) = get_log_system() {
        log_system.remove_subscriber(window.label());
        Ok(())
    } else {
        Err("日志系统未初始化".to_string())
    }
}

/// 清空日志缓冲区
#[tauri::command]
pub async fn clear_logs(
    _state: State<'_, crate::app::AppState>
) -> Result<(), String> {
    log::info!("🗑️ 清空日志缓冲区");
    
    if let Some(log_system) = get_log_system() {
        let mut buffer = log_system.buffer.write().unwrap();
        *buffer = crate::app::logging::CircularBuffer::new(buffer.capacity);
        Ok(())
    } else {
        Err("日志系统未初始化".to_string())
    }
}

/// 获取日志统计信息
#[tauri::command]
pub async fn get_log_stats(
    _state: State<'_, crate::app::AppState>
) -> Result<LogStats, String> {
    log::info!("📊 获取日志统计信息");
    
    if let Some(log_system) = get_log_system() {
        let buffer = log_system.buffer.read().unwrap();
        let recent_logs = buffer.get_recent(buffer.len());
        
        let mut stats = LogStats::default();
        stats.total_logs = recent_logs.len();
        
        for log in recent_logs {
            match log.level {
                crate::app::logging::LogLevel::Error => stats.error_count += 1,
                crate::app::logging::LogLevel::Warn => stats.warn_count += 1,
                crate::app::logging::LogLevel::Info => stats.info_count += 1,
                crate::app::logging::LogLevel::Debug => stats.debug_count += 1,
                crate::app::logging::LogLevel::Trace => stats.trace_count += 1,
            }
        }
        
        Ok(stats)
    } else {
        Err("日志系统未初始化".to_string())
    }
}

/// 日志统计信息
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct LogStats {
    pub total_logs: usize,
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub debug_count: usize,
    pub trace_count: usize,
}