// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod capture;
mod auth;
use log::{error, info};
mod cmd;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = tauri::Manager::get_webview_window(_app, "main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
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
            cmd::get_token_event_history
        ])
        .setup(|_app| {// 初始化 AppHandle
            if let Err(e) = capture::init_app_handle(_app.handle().clone()) {
                error!("初始化 AppHandle 失败: {}", e);
            }
            
            // 初始化认证系统
            if let Err(e) = auth::init_auth_system() {
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

            
            
            // 注意：不再自动启动数据包捕获，而是由用户点击按钮触发
            info!("应用已启动，等待用户请求开始捕获");
            
            Ok(())
        })
        .on_window_event(|_event_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                info!("窗口关闭，停止数据包捕获");
                // if let Err(e) = capture::stop_capture() {
                //     error!("停止数据包捕获失败: {}", e);
                // }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
