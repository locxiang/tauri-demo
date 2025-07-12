// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
mod service;
use log::{error, info};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    
    info!("🚀 启动数字重庆业务数据巡查自动化系统 v{}", env!("CARGO_PKG_VERSION"));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![
            api::get_capture_status,
            api::set_status_channel,
            api::set_http_channel,
            api::init_capture,
            api::stop_capture,
            api::has_pcap,
            api::get_network_devices,
            api::create_packet_window,
            // Auth系统命令
            api::get_all_token_status,
            api::get_system_token,
            api::clear_system_token,
            api::clear_all_tokens,
            api::set_token_event_channel,
            // 日志系统命令
            api::get_recent_logs,
            api::get_total_log_count,
            api::clear_logs,
            api::subscribe_log_stream,
            api::add_test_log,
            api::start_test_log_generator,
            // 工具命令
            api::open_devtools,
            api::open_folder,
            // 文件匹配命令
            api::find_similar_files,
        ])
        .setup(|app| {
            // 初始化日志管理器基础组件（同步）
            service::logread::LogManager::init();
            
            // 初始化 AppHandle
            if let Err(e) = api::init_app_handle(app.handle().clone()) {
                error!("初始化 AppHandle 失败: {}", e);
            }
            if let Err(e) = api::init_capture_system() {
                error!("初始化捕获系统失败: {}", e);
            }   
            
            // 使用新的初始化流程
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            
            // 在运行时上下文中初始化异步组件
            if let Err(e) = rt.block_on(async {
                // 初始化日志系统异步组件
                if let Err(e) = service::logread::LogManager::init_async().await {
                    return Err(anyhow::anyhow!("日志系统异步初始化失败: {}", e));
                }
                
                // 初始化认证系统
                api::init_auth_system().await
            }) {
                error!("❌ 异步系统初始化失败: {}", e);
            } else {
                info!("🔐 所有系统初始化成功");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


