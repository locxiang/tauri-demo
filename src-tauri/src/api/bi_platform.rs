use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;
use anyhow::{Result, anyhow};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct BiQueryRequest {
    pub olap_query_param: String,
    pub component_id: String,
    pub component_type: String,
    pub report_id: String,
    pub last_refresh_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiQueryResponse {
    pub success: bool,
    pub data: Option<Value>,
    pub message: Option<String>,
    pub error_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BiPlatformConfig {
    pub base_url: String,
    pub csrf_token: String,
    pub cookies: HashMap<String, String>,
}

impl BiPlatformConfig {
    pub fn new() -> Self {
        let mut cookies = HashMap::new();
        cookies.insert("qbi_locale".to_string(), "zh-CN".to_string());
        cookies.insert("csrf_token".to_string(), "6347d38f-8a70-4e50-b331-7e4d7a1677cd".to_string());
        cookies.insert("x_login_ck".to_string(), "57dd805b-2525-40dc-97fa-b6e70a71249c".to_string());
        cookies.insert("x_login_pk".to_string(), "03239304ecc1472190e88f75f3d21704".to_string());

        Self {
            base_url: "http://23.210.227.16".to_string(),
            csrf_token: "6347d38f-8a70-4e50-b331-7e4d7a1677cd".to_string(),
            cookies,
        }
    }
}

pub struct BiPlatformService {
    client: Client,
    config: BiPlatformConfig,
}

impl BiPlatformService {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            config: BiPlatformConfig::new(),
        })
    }

    pub async fn query_by_param(&self, request: BiQueryRequest) -> Result<BiQueryResponse> {
        let url = format!("{}/api/v2/biPlatform/query/byQueryParam", self.config.base_url);
        
        // 构建请求头
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Connection", "keep-alive".parse()?);
        headers.insert("Cache-Control", "max-age=0".parse()?);
        headers.insert("X-GW-Referer", format!("{}/report/view.htm?id={}", 
            self.config.base_url, request.report_id).parse()?);
        headers.insert("x-csrf-token", self.config.csrf_token.parse()?);
        headers.insert("X-Requested-With", "XMLHttpRequest".parse()?);
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.131 Safari/537.36 Edg/92.0.902.67".parse()?);
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse()?);
        headers.insert("Accept", "*/*".parse()?);
        headers.insert("Origin", self.config.base_url.parse()?);
        headers.insert("Referer", format!("{}/report/view.htm?id={}", 
            self.config.base_url, request.report_id).parse()?);
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse()?);

        // 构建Cookie字符串
        let cookie_str = self.config.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ");
        headers.insert("Cookie", cookie_str.parse()?);

        // 构建请求体
        let mut form_data = HashMap::new();
        form_data.insert("olapQueryParam".to_string(), request.olap_query_param);
        form_data.insert("componentId".to_string(), request.component_id);
        form_data.insert("componentType".to_string(), request.component_type);
        form_data.insert("reportId".to_string(), request.report_id);
        form_data.insert("lastRefreshTime".to_string(), request.last_refresh_time.to_string());

        // 发送请求
        let response = self.client
            .post(&url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let response_data: Value = response.json().await?;
            
            // 解析响应
            if let Some(success) = response_data.get("success").and_then(|v| v.as_bool()) {
                Ok(BiQueryResponse {
                    success,
                    data: response_data.get("data").cloned(),
                    message: response_data.get("message").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    error_code: response_data.get("errorCode").and_then(|v| v.as_str()).map(|s| s.to_string()),
                })
            } else {
                // 如果响应格式不符合预期，返回原始数据
                Ok(BiQueryResponse {
                    success: true,
                    data: Some(response_data),
                    message: None,
                    error_code: None,
                })
            }
        } else {
            Err(anyhow!("HTTP请求失败: {}", response.status()))
        }
    }
}

// Tauri命令：发送BI平台查询请求
#[tauri::command]
pub async fn send_bi_query(
    state: State<'_, BiPlatformService>,
) -> Result<BiQueryResponse, String> {
    // 使用默认的查询参数
    let request = BiQueryRequest {
        olap_query_param: "%7B%22componentId%22%3A%22gs5yyp7s%22%2C%22componentName%22%3A%22Sheet1%21A1%22%2C%22configs%22%3A%5B%7B%22type%22%3A%22field%22%2C%22config%22%3A%7B%22fields%22%3A%5B%7B%22guid%22%3A%22cf1b3b67-1211-4059-a358-b5f40789602c%22%2C%22fid%22%3A%22bbafe16af3%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22a2bb19a4-9a69-4b3e-bfad-e37e15bf14c2%22%2C%22fid%22%3A%2292fa3dd229%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22c2e713ec-d165-4c17-af16-7ad96e5345a1%22%2C%22fid%22%3A%227e954d6c5d%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%2251873768-bd11-4d9d-8273-737b02ce1c0c%22%2C%22fid%22%3A%22e32e23ee19%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22ab8929e7-6b97-475c-a1ac-bfed2e9bfa58%22%2C%22fid%22%3A%22c6da58ef1c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%2289e6c818-52be-44f1-be61-7e0bba069fe9%22%2C%22fid%22%3A%229d4db0ec2c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%22d683ebd5-e05c-4de6-a8c5-28a810fdf626%22%2C%22fid%22%3A%223ac9584256%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%224957b23a-3cf0-4275-ad88-4db6011ea1d5%22%2C%22fid%22%3A%22bfa140936c%22%2C%22areaType%22%3A%22row%22%7D%2C%7B%22guid%22%3A%221fc65896-5d91-4f06-949f-797b7b1f4106%22%2C%22fid%22%3A%22e783143093%22%2C%22areaType%22%3A%22row%22%7D%5D%7D%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%7D%2C%7B%22type%22%3A%22paging%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22limit%22%3A30000%2C%22offset%22%3A0%2C%22pagedByAllDim%22%3Atrue%7D%7D%2C%7B%22type%22%3A%22queryConfig%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22needCount%22%3Atrue%2C%22queryCount%22%3Atrue%2C%22queryDetail%22%3Atrue%7D%7D%2C%7B%22type%22%3A%22advancedParam%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22autoInsightParam%22%3A%7B%22enable%22%3Afalse%7D%2C%22wordCloudParam%22%3A%7B%7D%2C%22summarizeParams%22%3A%5B%5D%2C%22trendLineParams%22%3A%5B%5D%2C%22forecastParams%22%3A%5B%5D%2C%22anomalyDetectionParams%22%3A%5B%5D%2C%22clusteringParams%22%3A%5B%5D%2C%22groupParam%22%3Anull%7D%7D%2C%7B%22type%22%3A%22annotationParam%22%2C%22cubeId%22%3A%223cf5deaa-db7c-4700-9cae-07f68759cb1c%22%2C%22config%22%3A%7B%22measureThresholdParams%22%3A%5B%5D%2C%22inflectionPointParams%22%3A%5B%5D%7D%7D%5D%2C%22dataType%22%3A%22general%22%2C%22reportId%22%3A%2274271567-0203-4ff7-b43f-6195e598a8b1%22%7D".to_string(),
        component_id: "gs5yyp7s".to_string(),
        component_type: "77".to_string(),
        report_id: "74271567-0203-4ff7-b43f-6195e598a8b1".to_string(),
        last_refresh_time: chrono::Utc::now().timestamp() as f64,
    };
    
    state.query_by_param(request)
        .await
        .map_err(|e| e.to_string())
}

// 初始化BI平台服务
pub fn init_bi_platform_service() -> BiPlatformService {
    BiPlatformService::new()
        .expect("Failed to initialize BI Platform service")
} 