// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod capture;
mod auth;
use log::{error, info};
mod cmd;
mod logread;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets() // 清除所有默认目标，避免产生多个日志文件
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("app_logs".to_string()),
                    },
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                )) // 保留控制台输出用于调试
                .max_file_size(10_000_000) // 10MB
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(log::LevelFilter::Info)
                .level_for("big_data_rpa_v4", log::LevelFilter::Debug)
                .level_for("tauri_app_lib", log::LevelFilter::Debug) // 为应用模块设置更详细的日志级别
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d"),
                        chrono::Local::now().format("%H:%M:%S"),
                        record.level().to_string().to_uppercase(),
                        record.module_path().unwrap_or("unknown"),
                        message
                    ))
                })
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build()
        )
        .invoke_handler(tauri::generate_handler![
            cmd::get_capture_status,
            cmd::set_status_channel,
            cmd::set_http_channel,
            cmd::init_capture,
            cmd::stop_capture,
            cmd::has_pcap,
            cmd::get_network_devices,
            cmd::create_packet_window,
            cmd::focus_packet_window,
            // Auth系统命令
            cmd::get_all_token_status,
            cmd::get_system_token,
            cmd::clear_system_token,
            cmd::clear_all_tokens,
            cmd::set_token_event_channel,
            // 日志系统命令
            logread::get_system_logs,
            cmd::open_devtools,
            cmd::open_folder,
        ])
        .setup(|app| {
            // 在Tauri的setup中，应该已经有Tokio运行时上下文
            // 所以我们可以直接使用tokio::spawn
            
            // 初始化 AppHandle
            if let Err(e) = capture::init_app_handle(app.handle().clone()) {
                error!("初始化 AppHandle 失败: {}", e);
            }
            if let Err(e) = capture::init_capture_system() {
                error!("初始化捕获系统失败: {}", e);
            }   
            
            // 初始化认证系统
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            if let Err(e) = rt.block_on(auth::init_auth_system()) {
                error!("初始化认证系统失败: {}", e);
            } else {
                info!("🔐 认证系统初始化成功");
            }
            
            {
                if capture::has_capture_prerequisites() {
                    #[cfg(target_os = "macos")]
                    info!("检测到ChmodBPF已安装，可以直接使用抓包功能");
                    
                    #[cfg(target_os = "windows")]
                    info!("检测到Npcap已安装，可以直接使用抓包功能");
                } else {
                    #[cfg(target_os = "macos")]
                    info!("未检测到ChmodBPF，抓包功能可能受限");

                    #[cfg(target_os = "windows")]
                    info!("未检测到Npcap，抓包功能可能受限");
                }
            }
            
            Ok(())
        })
        // .on_window_event(|event_window, event| {
        //     if let tauri::WindowEvent::CloseRequested { _api, .. } = event {
        //         info!("用户请求关闭窗口")
        //     }
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}