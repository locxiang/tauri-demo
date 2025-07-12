// 创建数据包详情窗口
#[tauri::command]
pub async fn create_packet_window(
    app: tauri::AppHandle,
    title: String,
    url: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    println!("创建数据包详情窗口: {}", url);
    // 创建新窗口
    tauri::WebviewWindowBuilder::new(
        &app,
        &url,
        tauri::WebviewUrl::App(url.clone().into())
    )
    .title(&title)
    .inner_size(width, height)
    .center()
    .resizable(false)
    .minimizable(true)
    .maximizable(false)
    // .always_on_top(true) // 设置窗口始终在最前面
    .build()
    .map_err(|e| e.to_string())?;
        
    Ok(())
}