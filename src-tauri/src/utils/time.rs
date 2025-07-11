use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 获取当前时间戳（毫秒）
pub fn current_timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// 格式化时间戳为可读字符串
pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| chrono::Utc::now());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 格式化持续时间
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}秒", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        if remaining_seconds == 0 {
            format!("{}分钟", minutes)
        } else {
            format!("{}分钟{}秒", minutes, remaining_seconds)
        }
    } else {
        let hours = seconds / 3600;
        let remaining_minutes = (seconds % 3600) / 60;
        if remaining_minutes == 0 {
            format!("{}小时", hours)
        } else {
            format!("{}小时{}分钟", hours, remaining_minutes)
        }
    }
}

/// 计算两个时间戳之间的差值（秒）
pub fn time_diff(start: u64, end: u64) -> u64 {
    if end > start {
        end - start
    } else {
        0
    }
}

/// 检查时间戳是否在指定范围内
pub fn is_timestamp_in_range(timestamp: u64, start: u64, end: u64) -> bool {
    timestamp >= start && timestamp <= end
}