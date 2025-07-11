use regex::Regex;

/// 验证URL格式
pub fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(url)
}

/// 验证IP地址格式
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

/// 验证端口号
pub fn is_valid_port(port: u16) -> bool {
    port != 0
}

/// 验证Token格式
pub fn is_valid_token(token: &str) -> bool {
    // 基本验证：长度大于10，包含字母数字
    if token.len() < 10 {
        return false;
    }
    
    // 检查是否包含危险字符
    let dangerous_chars = ['\n', '\r', '\0', ';', '\'', '"', '<', '>'];
    for char in dangerous_chars {
        if token.contains(char) {
            return false;
        }
    }
    
    true
}

/// 验证系统ID格式
pub fn is_valid_system_id(system_id: &str) -> bool {
    let system_id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    system_id_regex.is_match(system_id) && system_id.len() >= 3 && system_id.len() <= 50
}

/// 验证Cookie值格式
pub fn is_valid_cookie_value(cookie_value: &str) -> bool {
    // Cookie值不能包含某些特殊字符
    let invalid_chars = [';', '\n', '\r', '\0'];
    for char in invalid_chars {
        if cookie_value.contains(char) {
            return false;
        }
    }
    
    // 长度检查
    cookie_value.len() <= 4096
}

/// 验证文件路径
pub fn is_valid_file_path(path: &str) -> bool {
    // 基本路径验证
    if path.is_empty() || path.len() > 1024 {
        return false;
    }
    
    // 检查是否包含危险字符
    let dangerous_chars = ['\0', '<', '>', '|', '"'];
    for char in dangerous_chars {
        if path.contains(char) {
            return false;
        }
    }
    
    true
}

/// 验证HTTP方法
pub fn is_valid_http_method(method: &str) -> bool {
    matches!(method.to_uppercase().as_str(), 
        "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH" | "TRACE" | "CONNECT"
    )
}

/// 验证HTTP状态码
pub fn is_valid_http_status_code(code: u16) -> bool {
    code >= 100 && code <= 599
}