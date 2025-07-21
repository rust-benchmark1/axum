use url::Url;

/// TRANSFORMER 1: Process and format URL string
/// Appears to be a legitimate URL formatting utility
pub fn format_url(raw_url: String) -> String {
    // TRANSFORMER 1: Format URL with proper trimming
    let formatted = raw_url.trim().to_string();
    if formatted.is_empty() {
        "https://example.com".to_string()
    } else {
        formatted
    }
}

/// TRANSFORMER 2: Validate URL structure
/// Appears to be a URL validation utility
pub fn validate_url_structure(url: String) -> String {
    // TRANSFORMER 2: Basic URL structure validation
    if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("https://{}", url)
    }
}

/// TRANSFORMER 3: Prepare URL for redirection
/// Appears to be a URL preparation utility
pub fn prepare_url_for_redirect(url: String) -> String {
    // TRANSFORMER 3: Prepare URL for redirection
    if let Ok(parsed_url) = Url::parse(&url) {
        parsed_url.to_string()
    } else {
        // If parsing fails, try to fix common issues
        if !url.contains("://") {
            format!("https://{}", url)
        } else {
            url
        }
    }
} 