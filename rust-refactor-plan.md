# Rust 代码重构方案

## 🎯 重构目标

### 当前问题
1. **模块职责不清晰** - `cmd.rs` 承担了太多不同领域的命令
2. **层次结构混乱** - 缺少清晰的分层架构  
3. **代码组织分散** - 相关功能分散在不同文件中
4. **缺少统一的错误处理** - 错误处理方式不统一
5. **缺少配置管理** - 配置散落在各处

### 重构原则
- **单一职责原则** - 每个模块专注于一个特定的功能领域
- **依赖倒置原则** - 高层模块不依赖低层模块
- **开闭原则** - 对扩展开放，对修改关闭
- **接口分离原则** - 使用细粒度接口
- **领域驱动设计** - 按业务领域组织代码

## 📁 新的目录结构

```
src-tauri/src/
├── main.rs                    # 程序入口
├── lib.rs                     # 库入口和应用初始化
├── app/                       # 应用层
│   ├── mod.rs                # 应用层模块入口
│   ├── config.rs             # 应用配置
│   ├── error.rs              # 统一错误类型
│   ├── state.rs              # 应用状态管理
│   └── setup.rs              # 应用初始化
├── api/                       # API层 (Tauri Commands)
│   ├── mod.rs                # API层模块入口
│   ├── capture.rs            # 网络抓包相关API
│   ├── auth.rs               # 认证管理相关API
│   ├── system.rs             # 系统相关API
│   └── window.rs             # 窗口管理相关API
├── domain/                    # 领域层
│   ├── mod.rs                # 领域层模块入口
│   ├── capture/              # 网络抓包领域
│   │   ├── mod.rs
│   │   ├── entities.rs       # 实体定义
│   │   ├── services.rs       # 领域服务
│   │   ├── repository.rs     # 仓储接口
│   │   └── events.rs         # 领域事件
│   ├── auth/                 # 认证管理领域
│   │   ├── mod.rs
│   │   ├── entities.rs       # Token、System等实体
│   │   ├── services.rs       # 认证服务
│   │   ├── repository.rs     # Token存储接口
│   │   ├── events.rs         # Token事件
│   │   └── systems/          # 各业务系统实现
│   │       ├── mod.rs
│   │       ├── traits.rs     # 系统认证trait
│   │       ├── registry.rs   # 系统注册
│   │       ├── test_system.rs
│   │       ├── bi_system.rs
│   │       ├── three_system.rs
│   │       └── drs_system.rs
│   └── shared/               # 共享领域
│       ├── mod.rs
│       ├── value_objects.rs  # 值对象
│       └── events.rs         # 共享事件
├── infrastructure/            # 基础设施层
│   ├── mod.rs                # 基础设施层模块入口
│   ├── network/              # 网络基础设施
│   │   ├── mod.rs
│   │   ├── pcap.rs           # pcap网络抓包
│   │   ├── device.rs         # 网络设备管理
│   │   └── protocol.rs       # 协议解析
│   ├── storage/              # 存储基础设施
│   │   ├── mod.rs
│   │   ├── memory.rs         # 内存存储
│   │   ├── file.rs           # 文件存储
│   │   └── cache.rs          # 缓存实现
│   ├── logging/              # 日志基础设施
│   │   ├── mod.rs
│   │   ├── logger.rs         # 日志器
│   │   └── reader.rs         # 日志读取
│   └── system/               # 系统基础设施
│       ├── mod.rs
│       ├── permissions.rs    # 权限检查
│       ├── platform.rs       # 平台特定代码
│       └── process.rs        # 进程管理
├── utils/                     # 工具模块
│   ├── mod.rs
│   ├── crypto.rs             # 加密工具
│   ├── time.rs               # 时间工具
│   ├── validation.rs         # 验证工具
│   └── parser.rs             # 解析工具
└── tests/                     # 测试模块
    ├── mod.rs
    ├── integration/          # 集成测试
    └── unit/                 # 单元测试
```

## 🏗️ 分层架构设计

### 1. 应用层 (app/)
**职责**: 应用程序的启动、配置管理、全局状态管理

```rust
// app/mod.rs
pub mod config;
pub mod error;
pub mod state;
pub mod setup;

pub use error::{AppError, AppResult};
pub use config::AppConfig;
pub use state::AppState;
```

### 2. API层 (api/)
**职责**: 处理来自前端的Tauri命令，协调领域服务

```rust
// api/mod.rs
pub mod capture;
pub mod auth;
pub mod system;
pub mod window;

// 所有API的统一入口
pub use capture::*;
pub use auth::*;
pub use system::*;
pub use window::*;
```

### 3. 领域层 (domain/)
**职责**: 核心业务逻辑，不依赖外部框架

```rust
// domain/mod.rs
pub mod capture;
pub mod auth;
pub mod shared;

// 重新导出重要的领域类型
pub use capture::entities::*;
pub use auth::entities::*;
pub use shared::value_objects::*;
```

### 4. 基础设施层 (infrastructure/)
**职责**: 具体的技术实现，如网络、存储、日志等

```rust
// infrastructure/mod.rs
pub mod network;
pub mod storage;
pub mod logging;
pub mod system;

// 重新导出重要的基础设施服务
pub use network::pcap::PcapService;
pub use storage::memory::MemoryTokenStore;
pub use logging::logger::Logger;
```

## 🔧 核心模块重构

### 1. 统一错误处理
```rust
// app/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("网络错误: {0}")]
    Network(#[from] NetworkError),
    
    #[error("认证错误: {0}")]
    Auth(#[from] AuthError),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Tauri错误: {0}")]
    Tauri(#[from] tauri::Error),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("设备不存在: {0}")]
    DeviceNotFound(String),
    
    #[error("权限不足")]
    PermissionDenied,
    
    #[error("抓包失败: {0}")]
    CaptureError(String),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token无效")]
    InvalidToken,
    
    #[error("系统未注册: {0}")]
    SystemNotRegistered(String),
    
    #[error("Token已过期")]
    TokenExpired,
}

pub type AppResult<T> = Result<T, AppError>;
```

### 2. 配置管理
```rust
// app/config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub auth: AuthConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub capture_buffer_size: usize,
    pub capture_timeout: u64,
    pub max_packet_size: usize,
    pub filter_expression: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub token_refresh_interval: u64,
    pub token_expire_threshold: u64,
    pub max_token_cache_size: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub max_file_size: u64,
    pub max_files: usize,
    pub console_output: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                capture_buffer_size: 1_000_000,
                capture_timeout: 1000,
                max_packet_size: 65535,
                filter_expression: "tcp port 80 or tcp port 8080 or tcp port 443".to_string(),
            },
            auth: AuthConfig {
                token_refresh_interval: 60,
                token_expire_threshold: 300,
                max_token_cache_size: 100,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                max_file_size: 10_000_000,
                max_files: 5,
                console_output: true,
            },
        }
    }
}
```

### 3. 应用状态管理
```rust
// app/state.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::capture::services::CaptureService;
use crate::domain::auth::services::AuthService;
use crate::app::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub capture_service: Arc<CaptureService>,
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> crate::app::AppResult<Self> {
        let config = Arc::new(config);
        
        // 初始化服务
        let capture_service = Arc::new(CaptureService::new(config.clone()).await?);
        let auth_service = Arc::new(AuthService::new(config.clone()).await?);
        
        Ok(Self {
            config,
            capture_service,
            auth_service,
        })
    }
}
```

### 4. API层重构
```rust
// api/capture.rs
use tauri::{State, ipc::Channel};
use crate::app::{AppState, AppResult};
use crate::domain::capture::entities::*;

#[tauri::command]
pub async fn get_capture_status(
    state: State<'_, AppState>
) -> Result<CaptureStatus, String> {
    state.capture_service
        .get_status()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn init_capture(
    device_name: String,
    state: State<'_, AppState>
) -> Result<(), String> {
    state.capture_service
        .start_capture(&device_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_capture(
    state: State<'_, AppState>
) -> Result<(), String> {
    state.capture_service
        .stop_capture()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_network_devices(
    state: State<'_, AppState>
) -> Result<Vec<NetworkDevice>, String> {
    state.capture_service
        .get_network_devices()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_http_channel(
    channel: Channel<HttpPacket>,
    state: State<'_, AppState>
) -> Result<(), String> {
    state.capture_service
        .set_http_channel(channel)
        .await
        .map_err(|e| e.to_string())
}
```

### 5. 领域服务重构
```rust
// domain/capture/services.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::app::{AppConfig, AppResult};
use crate::domain::capture::entities::*;
use crate::domain::capture::repository::CaptureRepository;
use crate::infrastructure::network::pcap::PcapService;

pub struct CaptureService {
    config: Arc<AppConfig>,
    pcap_service: Arc<PcapService>,
    repository: Arc<dyn CaptureRepository>,
    status: Arc<RwLock<CaptureStatus>>,
}

impl CaptureService {
    pub async fn new(config: Arc<AppConfig>) -> AppResult<Self> {
        let pcap_service = Arc::new(PcapService::new(config.clone())?);
        let repository = Arc::new(
            crate::infrastructure::storage::memory::MemoryCaptureRepository::new()
        );
        
        Ok(Self {
            config,
            pcap_service,
            repository,
            status: Arc::new(RwLock::new(CaptureStatus::default())),
        })
    }
    
    pub async fn start_capture(&self, device_name: &str) -> AppResult<()> {
        // 检查权限
        if !self.pcap_service.has_permissions()? {
            return Err(crate::app::NetworkError::PermissionDenied.into());
        }
        
        // 启动抓包
        self.pcap_service.start_capture(device_name).await?;
        
        // 更新状态
        let mut status = self.status.write().await;
        status.is_running = true;
        status.device_name = Some(device_name.to_string());
        
        Ok(())
    }
    
    pub async fn stop_capture(&self) -> AppResult<()> {
        self.pcap_service.stop_capture().await?;
        
        let mut status = self.status.write().await;
        status.is_running = false;
        status.device_name = None;
        
        Ok(())
    }
    
    pub async fn get_status(&self) -> AppResult<CaptureStatus> {
        Ok(self.status.read().await.clone())
    }
    
    pub async fn get_network_devices(&self) -> AppResult<Vec<NetworkDevice>> {
        self.pcap_service.get_network_devices().await
    }
}
```

## 🔄 迁移步骤

### 第一阶段：创建新结构
1. 创建新的目录结构
2. 定义统一的错误类型
3. 创建配置管理模块
4. 重构应用状态管理

### 第二阶段：迁移API层
1. 按功能拆分cmd.rs
2. 创建专门的API模块
3. 统一错误处理

### 第三阶段：重构领域层
1. 提取领域实体
2. 创建领域服务
3. 定义仓储接口

### 第四阶段：重构基础设施层
1. 抽象网络服务
2. 重构存储层
3. 统一日志处理

### 第五阶段：测试和优化
1. 添加单元测试
2. 添加集成测试
3. 性能优化

## 🎯 重构后的优势

### 1. 清晰的职责分离
- 每个模块都有明确的职责
- 降低了模块间的耦合度
- 提高了代码的可测试性

### 2. 更好的可维护性
- 统一的错误处理
- 集中的配置管理
- 清晰的依赖关系

### 3. 更强的扩展性
- 新功能易于添加
- 现有功能易于修改
- 支持插件化架构

### 4. 更高的代码质量
- 更好的类型安全
- 更清晰的代码结构
- 更完善的测试覆盖

你希望我开始实施这个重构方案吗？我可以从创建新的目录结构和基础模块开始。