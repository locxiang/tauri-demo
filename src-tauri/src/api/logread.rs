use crate::service::logread::{LogReadService, LogEntry};

// 获取系统日志
#[tauri::command]
pub fn get_system_logs() -> Result<Vec<LogEntry>, String> {
    LogReadService::get_system_logs()
}
