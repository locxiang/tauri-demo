use tauri::command;
use crate::service::file_match::{FileMatchService, RelatedFile};

/// BI数据文件匹配API - 查找相似文件
#[command]
pub async fn find_similar_files(
    folder_path: String,
    search_text: String,
    max_results: usize,
) -> Result<Vec<RelatedFile>, String> {
    // 参数验证
    if folder_path.trim().is_empty() {
        return Err("文件夹路径不能为空".to_string());
    }
    
    if search_text.trim().is_empty() {
        return Err("搜索文本不能为空".to_string());
    }
    
    if max_results == 0 {
        return Err("最大结果数量必须大于0".to_string());
    }
    
    if max_results > 10 {
        return Err("最大结果数量不能超过10".to_string());
    }
    
    // 调用Service层处理业务逻辑
    FileMatchService::find_similar_files(folder_path, search_text, max_results)
        .await
        .map_err(|e| format!("文件匹配失败: {}", e))
}