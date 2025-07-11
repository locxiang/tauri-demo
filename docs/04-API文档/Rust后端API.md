# Rust后端API

## 📋 API概述

基于Tauri架构的Rust后端API接口，采用分层架构设计，提供清晰的模块边界和统一的接口规范。

## 🏗️ API架构

### 分层设计
```
API层 (api/) → 应用层 (app/) → 领域层 (domain/) → 基础设施层 (infrastructure/)
```

### 模块导出
```rust
// api/mod.rs - 统一API导出
pub use capture::*;
pub use auth::*;
pub use system::*;
pub use window::*;
pub use logging::*;
```

## 📋 日志管理API

| 命令名称 | 功能描述 | 主要参数 | 返回值 |
|---------|---------|---------|-------|
| `get_recent_logs` | 获取历史日志 | `limit`, `filters` | `Vec<LogEntry>` |
| `subscribe_log_stream` | 订阅实时日志流 | `window`, `filters` | `()` |
| `unsubscribe_log_stream` | 取消订阅 | `window` | `()` |
| `clear_logs` | 清空日志缓冲区 | - | `()` |
| `get_log_stats` | 获取日志统计 | - | `LogStats` |

### 实时流事件
- **事件名称**: `"log-stream"`
- **事件数据**: `LogEntry`

### 使用示例
```typescript
// 订阅日志流
await invoke('subscribe_log_stream');
const unlisten = await listen<LogEntry>('log-stream', (event) => {
    console.log('新日志:', event.payload);
});

// 获取历史日志
const logs = await invoke<LogEntry[]>('get_recent_logs', { limit: 100 });
```

## 📡 网络捕获API

| 命令名称 | 功能描述 | 主要参数 | 返回值 |
|---------|---------|---------|-------|
| `get_capture_status` | 获取捕获状态 | - | `CaptureStatus` |
| `init_capture` | 开始网络捕获 | `device_name` | `()` |
| `stop_capture` | 停止网络捕获 | - | `()` |
| `get_network_devices` | 获取网络设备列表 | - | `Vec<NetworkDevice>` |
| `set_status_channel` | 设置状态通道 | `window` | `()` |
| `set_http_channel` | 设置HTTP通道 | `window` | `()` |

### 数据类型
```rust
pub struct CaptureStatus {
    pub is_running: bool,           // 是否正在运行
    pub packets_captured: u64,      // 已捕获包数量
    pub device_name: Option<String>, // 设备名称
}

pub struct NetworkDevice {
    pub name: String,               // 设备名称
    pub description: Option<String>, // 设备描述
    pub is_up: bool,               // 是否启用
    pub addresses: Vec<String>,     // IP地址列表
}
```

## 🔐 认证管理API

| 命令名称 | 功能描述 | 主要参数 | 返回值 |
|---------|---------|---------|-------|
| `get_all_token_status` | 获取所有Token状态 | - | `Vec<TokenStatus>` |
| `get_system_token` | 获取系统Token | `system_id` | `Option<String>` |
| `clear_system_token` | 清除系统Token | `system_id` | `()` |
| `clear_all_tokens` | 清除所有Token | - | `()` |
| `refresh_token_status` | 刷新Token状态 | - | `Vec<TokenStatus>` |
| `set_token_event_channel` | 设置Token事件通道 | `window` | `()` |

### 数据类型
```rust
pub struct TokenStatus {
    pub system_id: String,          // 系统ID
    pub system_name: String,        // 系统名称
    pub has_token: bool,            // 是否有Token
    pub is_expired: bool,           // 是否已过期
}

pub struct TokenEvent {
    pub event_type: String,         // 事件类型
    pub system_id: String,          // 系统ID
    pub timestamp: u64,             // 时间戳
    pub details: String,            // 事件详情
}
```

## 🖥️ 系统管理API

| 命令名称 | 功能描述 | 主要参数 | 返回值 |
|---------|---------|---------|-------|
| `get_app_config` | 获取应用配置 | - | `AppConfig` |
| `update_app_config` | 更新应用配置 | `config` | `()` |
| `check_system_permissions` | 检查系统权限 | - | `PermissionStatus` |
| `get_system_info` | 获取系统信息 | - | `SystemInfo` |
| `has_pcap` | 检查pcap支持 | - | `bool` |

## 🪟 窗口管理API

| 命令名称 | 功能描述 | 主要参数 | 返回值 |
|---------|---------|---------|-------|
| `open_devtools` | 打开开发者工具 | - | `()` |
| `open_folder` | 打开文件夹 | `path` | `()` |
| `create_packet_window` | 创建数据包窗口 | `packet_data` | `()` |
| `close_window` | 关闭窗口 | `label` | `()` |
| `get_all_windows` | 获取所有窗口 | - | `Vec<WindowInfo>` |

## 🛠️ 通用数据类型

### LogEntry (日志条目)
```rust
pub struct LogEntry {
    pub id: u64,                    // 唯一ID
    pub timestamp: i64,             // 时间戳(毫秒)
    pub level: LogLevel,            // 日志级别
    pub target: String,             // 目标模块
    pub message: String,            // 日志消息
    pub file: Option<String>,       // 文件名
    pub line: Option<u32>,          // 行号
}
```

### LogFilters (日志过滤器)
```rust
pub struct LogFilters {
    pub level: Option<LogLevel>,        // 按级别过滤
    pub keywords: Option<Vec<String>>,  // 关键词过滤
    pub targets: Option<Vec<String>>,   // 目标模块过滤
    pub since: Option<i64>,             // 开始时间
    pub until: Option<i64>,             // 结束时间
}
```

## ⚠️ 错误处理

### 统一错误类型
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("初始化错误: {0}")]
    InitializationError(String),
    #[error("网络错误: {0}")]
    NetworkError(String),
    #[error("认证错误: {0}")]
    AuthError(String),
}
```

### 错误处理模式
API命令统一返回`Result<T, String>`，错误信息通过字符串返回给前端。

## 🔧 使用示例

### 完整流程示例
```typescript
// 1. 获取历史日志
const logs = await invoke<LogEntry[]>('get_recent_logs', { limit: 100 });

// 2. 订阅实时日志流
await invoke('subscribe_log_stream');
const unlisten = await listen<LogEntry>('log-stream', (event) => {
    updateLogDisplay(event.payload);
});

// 3. 网络捕获
const devices = await invoke<NetworkDevice[]>('get_network_devices');
await invoke('set_status_channel');
await invoke('init_capture', { device_name: devices[0].name });

// 4. 清理资源
await invoke('unsubscribe_log_stream');
await invoke('stop_capture');
unlisten();
```

## 📊 性能考虑

### 优化策略
- 所有API命令都是异步的，避免阻塞
- 使用环形缓冲区限制内存使用
- 事件驱动减少轮询开销
- 快速失败机制避免资源泄漏

### 调用频率建议
- `get_recent_logs`: 按需调用
- `subscribe_log_stream`: 应用启动时订阅一次
- `get_capture_status`: 定时调用，建议1-5秒间隔
- `get_log_stats`: 可频繁调用，开销较小

## 📋 版本历史

### v0.7.0 (当前)
- ✅ 新增现代化日志系统API
- ✅ 重构API层架构，模块化设计
- ✅ 统一错误处理机制
- ✅ 优化实时数据推送性能

### v0.6.0
- ✅ 基础网络捕获API
- ✅ 认证管理API
- ✅ 系统权限检查API