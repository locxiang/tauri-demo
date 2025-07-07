use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
// 日志条目结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}


// 获取系统日志
#[tauri::command]
pub fn get_system_logs() -> Result<Vec<LogEntry>, String> {
    let log_dir = get_log_directory();
    let log_file = log_dir.join("app_logs.log");
    
    log::info!("尝试读取日志文件: {:?}", log_file);
    
    if !log_file.exists() {
        log::warn!("日志文件不存在: {:?}", log_file);
        return Ok(vec![]);
    }
    
    // 读取日志文件
    let content = fs::read_to_string(&log_file)
        .map_err(|e| {
            let error_msg = format!("读取日志文件失败: {}", e);
            log::error!("{}", error_msg);
            error_msg
        })?;
    
    let all_lines: Vec<&str> = content.lines().collect();
    let total_lines = all_lines.len();
    log::info!("日志文件包含 {} 行", total_lines);
    
    // 获取最新的 1000 行（如果文件行数少于1000，则获取全部）
    let lines_to_process = if total_lines > 1000 {
        &all_lines[total_lines - 1000..]
    } else {
        &all_lines[..]
    };
    
    log::info!("读取最新的 {} 行进行解析", lines_to_process.len());
    
    let mut logs = Vec::new();
    let mut parsed_count = 0;
    
    // 解析最新的1000行日志内容
    for (line_index, line) in lines_to_process.iter().enumerate() {
        if let Some(log_entry) = parse_log_line(line) {
            logs.push(log_entry);
            parsed_count += 1;
        } else if !line.trim().is_empty() {
            log::debug!("无法解析行 {}: {}", total_lines - lines_to_process.len() + line_index + 1, line);
        }
    }
    
    log::info!("成功解析 {} / {} 行日志", parsed_count, lines_to_process.len());
    
    // 按时间戳正序排列（最新的在后面）
    logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    Ok(logs)
}

// 获取日志目录
fn get_log_directory() -> PathBuf {
    // 根据不同平台获取日志目录
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join("Library").join("Logs").join("com.big-data-rpa-v4.my")
    }
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
        PathBuf::from(local_app_data).join("com.big-data-rpa-v4.my").join("logs")
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let xdg_data_home = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            PathBuf::from(&home).join(".local").join("share").to_string_lossy().to_string()
        });
        PathBuf::from(xdg_data_home).join("com.big-data-rpa-v4.my").join("logs")
    }
}

// 解析单行日志
fn parse_log_line(line: &str) -> Option<LogEntry> {
    // 实际日志格式: [2025-07-07][18:57:24][INFO][tauri_app_lib::auth] 🚀 开始初始化Token认证系统...
    
    if line.is_empty() || !line.starts_with('[') {
        return None;
    }
    
    // 分割所有的 [...] 部分
    let mut brackets = Vec::new();
    let mut remaining = line;
    
    while remaining.starts_with('[') {
        if let Some(end) = remaining.find(']') {
            let content = &remaining[1..end];
            brackets.push(content);
            remaining = &remaining[end + 1..];
        } else {
            break;
        }
    }
    
    // 至少需要4个bracket: [date][time][level][module]
    if brackets.len() < 4 {
        return None;
    }
    
    let date = brackets[0];
    let time = brackets[1];
    let level = brackets[2].to_lowercase();
    let module = brackets[3];
    
    // 解析时间戳
    let datetime_str = format!("{} {}", date, time);
    let timestamp = parse_datetime(&datetime_str).unwrap_or_else(|| chrono::Utc::now().timestamp_millis() as u64);
    
    // 提取消息内容（去掉开头的空格）
    let message = remaining.trim().to_string();
    
    // 从模块信息中提取文件和行号
    let (file, line_num) = if module.contains("::") {
        // 将模块路径转换为文件路径格式
        let file_path = module.replace("::", "/") + ".rs";
        (Some(file_path), None)
    } else {
        (Some(module.to_string()), None)
    };
    
    Some(LogEntry {
        timestamp,
        level,
        message,
        file,
        line: line_num,
    })
}

// 解析日期时间字符串
fn parse_datetime(datetime_str: &str) -> Option<u64> {
    // 尝试解析 "2025-07-07 18:57:24" 格式
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        Some(dt.and_utc().timestamp_millis() as u64)
    } else {
        None
    }
}
