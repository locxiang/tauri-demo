use crate::service::logread::{self, LogEntry};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_recent_logs(limit: Option<usize>) -> Result<Vec<LogEntry>, String> {
    logread::get_recent_logs(limit).await
}

#[tauri::command]
pub async fn get_total_log_count() -> Result<u64, String> {
    logread::get_total_log_count().await
}

#[tauri::command]
pub async fn clear_logs() -> Result<(), String> {
    logread::clear_logs().await
}

#[tauri::command]
pub async fn subscribe_log_stream(app_handle: AppHandle) -> Result<(), String> {
    logread::subscribe_log_stream(app_handle).await
}

#[tauri::command] 
pub async fn add_test_log(level: String, message: String) -> Result<(), String> {
    logread::add_test_log(level, message).await
}

#[tauri::command] 
pub async fn start_test_log_generator() -> Result<(), String> {
    logread::start_test_log_generator().await
}
