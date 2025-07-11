// 应用核心模块
pub mod app;
pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod utils;


use log::{error, info};
use app::AppSetup;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化现代化日志系统
    if let Err(e) = app::init_log_system() {
        eprintln!("❌ 初始化日志系统失败: {}", e);
    } else {
        println!("✅ 现代化日志系统初始化成功");
    }
    
    info!("🚀 启动数字重庆业务数据巡查自动化系统 v{}", env!("CARGO_PKG_VERSION"));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![
            // 新的API接口
            api::get_capture_status,
            api::init_capture,
            api::stop_capture,
            api::get_network_devices,
            api::set_status_channel,
            api::set_http_channel,
            api::has_pcap,
            api::get_all_token_status,
            api::get_system_token,
            api::clear_system_token,
            api::clear_all_tokens,
            api::set_token_event_channel,
            api::refresh_token_status,
            api::get_app_config,
            api::update_app_config,
            api::get_permission_guide,
            api::check_system_permissions,
            api::get_system_info,
            api::open_devtools,
            api::open_folder,
            api::create_packet_window,
            api::focus_packet_window,
            api::close_window,
            api::get_all_windows,
            
            // 现代化日志系统命令
            api::get_recent_logs,
            api::subscribe_log_stream,
            api::unsubscribe_log_stream,
            api::clear_logs,
            api::get_log_stats,
        ])
        .setup(|app| {
            info!("🚀 启动数字重庆业务数据巡查自动化系统 v{}", env!("CARGO_PKG_VERSION"));
            info!("🔧 开始应用程序初始化...");
            
            // 使用新的初始化流程
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            
            match rt.block_on(async {
                // 初始化应用状态
                let app_state = AppSetup::initialize().await?;
                
                // 将应用状态存储到Tauri的状态管理中
                app.manage(app_state);
                
                
                Ok::<(), app::AppError>(())
            }) {
                Ok(_) => {
                    info!("✅ 应用程序初始化完成");
                }
                Err(e) => {
                    error!("❌ 应用程序初始化失败: {}", e);
                    return Err(Box::new(e));
                }
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


