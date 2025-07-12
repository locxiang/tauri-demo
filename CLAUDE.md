# Claude Code 协作配置

## 项目概况
- **项目名称**: 数字重庆业务数据巡查自动化系统 (Digital Chongqing Business Data Inspection Automation System)
- **公司**: 重庆秫米科技有限公司 (Chongqing Sumi Technology Co., Ltd.)
- **项目类型**: 政府系统数据监控RPA桌面软件

## 技术栈
- **前端**: Vue 3 + TypeScript + Vite + TailwindCSS + Pinia
- **后端**: Tauri + Rust + Tokio + async-trait
- **开发工具**: ESLint + Prettier + Vitest

## 常用开发命令

### 基础开发
```bash
pnpm dev          # 启动开发服务器 (前端:1420 + Vue DevTools:8098)
pnpm build        # 类型检查 + 构建前端
pnpm type-check   # TypeScript类型检查
cargo check       # Rust代码检查
cargo build       # Rust编译
pnpm bump         # 版本号管理
```

### Tauri相关
```bash
pnpm tauri dev    # 启动Tauri开发模式
pnpm tauri build  # 构建桌面应用
pnpm tauri dev --release  # 发布模式开发
```

### 调试工具
```bash
pnpm devtools     # 启动Vue DevTools
pnpm test         # 运行测试套件
```

## 项目架构设计

### 分层架构模式
这是一个采用**干净架构**的桌面RPA应用，严格分为两个核心层次：

```
┌─────────────────────────────────────┐
│             API 层                  │  
│  • 薄薄的接口层，仅处理参数转换    │
│  • Tauri命令处理                   │
│  • 错误格式化 (Result<T, String>)  │
├─────────────────────────────────────┤
│           Service 层               │
│  • 核心业务逻辑                   │
│  • 数据处理和状态管理              │
│  • 系统集成和外部通信              │
└─────────────────────────────────────┘
```

**核心功能模块**:
- **网络流量监控** (`service/capture.rs`): 实时HTTP包捕获和分析
- **多系统认证管理** (`service/auth/`): 政府业务系统的Token自动化管理  
- **日志系统** (`service/logread.rs`): 统一的日志收集和分析

## 编程风格和习惯

### 代码风格约定

#### 1. 注释和日志规范
- **所有注释使用中文**，确保代码可读性
- **日志消息使用中文 + Emoji表情符号**，便于快速识别操作类型:
  ```rust
  info!("🚀 开始初始化认证系统...");
  info!("✅ 认证系统初始化完成");
  error!("❌ 初始化失败: {}", e);
  debug!("🔄 处理HTTP请求: {} {}", method, path);
  warn!("⚠️  检测到过期Token");
  ```

#### 2. 错误处理模式
- **统一使用 `Result<T, String>` 作为API层返回类型**
- **错误转换模式**: `.map_err(|e| e.to_string())`
- **详细的错误信息**，包含中文描述和上下文

#### 3. 命名约定
- **函数名使用英文**，遵循Rust命名规范  
- **变量名和结构体使用英文**
- **注释、日志、错误信息使用中文**

### 架构模式和最佳实践

#### 1. 全局状态管理
使用 `OnceCell + Arc<Mutex<T>>` 模式管理全局状态:
```rust
static AUTH_SERVICE: OnceCell<Arc<AuthService>> = OnceCell::new();
static TOKEN_EVENT_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<TokenEvent>>>>> = OnceCell::new();
```

#### 2. 并发和异步模式
- **大量使用 async/await 和 Tokio 运行时**
- **无锁数据结构**: 优先使用 `DashMap` 等无锁集合
- **原子操作**: 使用 `AtomicBool`、`AtomicU64` 等原子类型

#### 3. 通道通信模式
- **前后端通信**: 使用 Tauri IPC `Channel<T>` 进行实时数据推送
- **全局通道存储**: 统一管理所有通道，支持模块间通信
- **事件驱动**: 通过事件机制解耦模块间依赖

#### 4. 数据结构设计
- **所有数据结构支持 Serde 序列化**
- **使用 `Arc<T>` 进行数据共享**，避免不必要的克隆
- **枚举类型使用 `#[serde(tag = "type")]` 进行标记**

#### 5. 模块组织原则
- **按功能垂直分割**: 每个功能模块独立，包含完整的业务逻辑
- **clear imports**: 明确的模块导入，避免循环依赖
- **工厂模式**: 用于创建可配置的系统实例 (如认证系统)

#### 6. 初始化和生命周期
- **分阶段初始化**: 按依赖关系顺序初始化各个系统
- **优雅的错误处理**: 初始化失败时提供详细的诊断信息
- **资源清理**: 适当的资源释放和清理机制

## 开发协作指南

### 新功能开发
1. **API层**: 在 `src/api/` 下创建对应模块，仅处理参数验证和格式转换
2. **Service层**: 在 `src/service/` 下实现具体业务逻辑
3. **注册命令**: 在 `lib.rs` 的 `invoke_handler` 中注册新的Tauri命令
4. **错误处理**: 统一使用 `Result<T, String>` 和中文错误信息

### 调试和日志
- **使用详细的中文日志**，包含操作前后状态
- **Emoji符号标记**: 🚀启动、✅成功、❌失败、🔄处理中、⚠️警告等
- **日志级别**: Debug用于详细跟踪，Info用于关键状态，Error用于失败情况

### 代码审查要点
- **架构分层是否清晰**
- **错误处理是否统一**
- **中文注释是否完整**
- **并发安全是否考虑**
- **资源管理是否适当**

## 沟通协作
- **使用中文进行技术讨论和代码交流**
- **代码注释和文档统一使用中文**
- **问题描述和解决方案使用中文表达**