use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelatedFile {
    pub path: String,
    pub similarity: f64,
}

/// 文件匹配服务
pub struct FileMatchService;

impl FileMatchService {
    /// 查找相似文件
    pub async fn find_similar_files(
        folder_path: String,
        search_text: String,
        max_results: usize,
    ) -> Result<Vec<RelatedFile>, String> {
        log::info!("🔍 开始搜索相似文件: 文件夹={}, 搜索文本={}", folder_path, search_text);
        
        let folder = Path::new(&folder_path);
        if !folder.exists() || !folder.is_dir() {
            return Err("指定的文件夹路径不存在或不是有效目录".to_string());
        }

        // 获取所有Excel文件
        let excel_files = Self::get_excel_files(folder)
            .map_err(|e| format!("遍历文件夹失败: {}", e))?;
        
        log::info!("📁 找到 {} 个Excel文件", excel_files.len());

        // 计算文本相似度并排序
        let mut similarities: Vec<RelatedFile> = excel_files
            .into_iter()
            .map(|file_path| {
                let file_display = Self::format_file_display(&file_path);

                log::info!("对比文件: {} 和 {}",search_text, file_display);
                let similarity = Self::calculate_similarity(&search_text, &file_display);
                
                RelatedFile {
                    path: file_path.to_string_lossy().to_string(),
                    similarity,
                }
            })
            .filter(|f| f.similarity > 0.0) // 只保留有相似度的文件
            .collect();

        // 按相似度降序排序
        similarities.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // 限制返回结果数量
        similarities.truncate(max_results);

        log::info!("✅ 返回 {} 个最相似的文件", similarities.len());
        Ok(similarities)
    }

    /// 递归获取文件夹中的所有Excel文件
    fn get_excel_files(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut excel_files = Vec::new();
        
        fn collect_excel_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
            let entries = fs::read_dir(dir)
                .map_err(|e| format!("读取目录失败: {}", e))?;
                
            for entry in entries {
                let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
                let path = entry.path();
                
                if path.is_dir() {
                    // 递归搜索子目录
                    collect_excel_files(&path, files)?;
                } else if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if ext == "xlsx" || ext == "xls" || ext == "csv" {
                        files.push(path);
                    }
                }
            }
            Ok(())
        }
        
        collect_excel_files(dir, &mut excel_files)?;
        Ok(excel_files)
    }

    /// 格式化文件显示名称（目录名/文件名）
    fn format_file_display(file_path: &std::path::Path) -> String {
        let parent_name = file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        format!("{}/{}", parent_name, file_name)
    }

    /// 计算两个字符串的相似度 (基于编辑距离的简化算法)
    fn calculate_similarity(text1: &str, text2: &str) -> f64 {
        // 预处理: 转换为小写并移除空格
        let s1 = text1.to_lowercase().replace(" ", "");
        let s2 = text2.to_lowercase().replace(" ", "");
        
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }
        
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        // 检查包含关系
        if s2.contains(&s1) || s1.contains(&s2) {
            let shorter = s1.len().min(s2.len()) as f64;
            let longer = s1.len().max(s2.len()) as f64;
            return shorter / longer;
        }

        // 计算Jaccard相似度 (基于字符2-gram)
        let bigrams1 = Self::get_char_bigrams(&s1);
        let bigrams2 = Self::get_char_bigrams(&s2);
        
        if bigrams1.is_empty() && bigrams2.is_empty() {
            return 1.0;
        }
        
        let intersection: HashSet<_> = bigrams1.intersection(&bigrams2).collect();
        let union: HashSet<_> = bigrams1.union(&bigrams2).collect();
        
        intersection.len() as f64 / union.len() as f64
    }

    /// 获取字符串的2-gram集合
    fn get_char_bigrams(text: &str) -> HashSet<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut bigrams = HashSet::new();
        
        for i in 0..chars.len().saturating_sub(1) {
            let bigram = format!("{}{}", chars[i], chars[i + 1]);
            bigrams.insert(bigram);
        }
        
        bigrams
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_calculation() {
        // 测试完全匹配
        assert_eq!(FileMatchService::calculate_similarity("test", "test"), 1.0);
        
        // 测试包含关系
        let sim = FileMatchService::calculate_similarity("教育", "重庆市教育委员会");
        assert!(sim > 0.0);
        
        // 测试部分相似
        let sim = FileMatchService::calculate_similarity("学校数据", "学校基础信息");
        assert!(sim > 0.0);
    }

    #[test] 
    fn test_bigrams() {
        let bigrams = FileMatchService::get_char_bigrams("测试");
        assert!(bigrams.contains("测试"));
    }
}