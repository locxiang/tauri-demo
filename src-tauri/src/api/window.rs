use tauri::{AppHandle, Manager};

/// 创建数据包详情窗口
#[tauri::command]
pub async fn create_packet_window(
    app: AppHandle,
    title: String,
    url: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    log::info!("🪟 创建数据包详情窗口: {}", title);
    
    // 创建新窗口
    tauri::WebviewWindowBuilder::new(
        &app,
        &url,
        tauri::WebviewUrl::App(url.clone().into())
    )
    .title(&title)
    .inner_size(width, height)
    .center()
    .resizable(true)
    .minimizable(true)
    .maximizable(false)
    .build()
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// 聚焦数据包详情窗口
#[tauri::command]
pub async fn focus_packet_window(
    app: AppHandle,
    window_label: String,
) -> Result<(), String> {
    log::info!("🔍 聚焦数据包详情窗口: {}", window_label);
    
    if let Some(window) = app.get_webview_window(&window_label) {
        window.set_focus().map_err(|e| e.to_string())?;
        window.unminimize().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// 关闭指定窗口
#[tauri::command]
pub async fn close_window(
    app: AppHandle,
    window_label: String,
) -> Result<(), String> {
    log::info!("❌ 关闭窗口: {}", window_label);
    
    if let Some(window) = app.get_webview_window(&window_label) {
        window.close().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// 获取所有窗口信息
#[tauri::command]
pub async fn get_all_windows(
    app: AppHandle,
) -> Result<Vec<serde_json::Value>, String> {
    log::info!("📋 获取所有窗口信息");
    
    let windows: Vec<serde_json::Value> = app.webview_windows()
        .into_iter()
        .map(|(label, window)| {
            serde_json::json!({
                "label": label,
                "title": window.title().unwrap_or_default(),
                "visible": window.is_visible().unwrap_or(true),
                "focused": window.is_focused().unwrap_or(false),
                "minimized": window.is_minimized().unwrap_or(false),
                "maximized": window.is_maximized().unwrap_or(false),
            })
        })
        .collect();
    
    Ok(windows)
}