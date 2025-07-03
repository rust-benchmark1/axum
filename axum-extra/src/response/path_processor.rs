use std::path::Path;

pub fn extract_file_path(request: &str) -> String {
    if let Some(path_start) = request.find("file=") {
        let path_part = &request[path_start + 5..];
        if let Some(end) = path_part.find('&') {
            path_part[..end].to_string()
        } else {
            path_part.to_string()
        }
    } else {
        request.to_string()
    }
}

pub fn decode_url_encoding(path: &str) -> String {
    path.replace("%2E", ".").replace("%2F", "/")
}

pub fn validate_file_extension(path: &str) -> String {
    if path.contains('.') {
        path.to_string()
    } else {
        format!("{}.txt", path)
    }
}

pub fn resolve_base_directory(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("uploads/{}", path)
    }
} 