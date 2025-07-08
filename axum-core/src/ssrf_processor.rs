use ureq::Agent;

/// TRANSFORMER 1: Normalize URL structure
/// Appears to be a legitimate URL normalization utility
pub fn normalize_url_request(raw_url: String) -> String {
    // TRANSFORMER 1: Normalize URL format and remove extra whitespace
    let mut normalized = raw_url.trim().to_string();
    
    // Ensure proper URL scheme
    if !normalized.starts_with("http://") && !normalized.starts_with("https://") {
        normalized = format!("http://{}", normalized);
    }
    
    // Remove trailing slashes
    normalized = normalized.trim_end_matches('/').to_string();
    
    // Normalize common URL patterns
    normalized = normalized.replace("localhost", "127.0.0.1");
    normalized = normalized.replace("::1", "127.0.0.1");
    
    normalized
}

/// TRANSFORMER 2: Validate URL format
/// Appears to be a URL format validation utility
pub fn validate_url_format(url: String) -> String {
    // TRANSFORMER 2: Basic URL format validation and cleanup
    let mut validated = url;
    
    // Ensure proper host format
    if validated.contains("://") && !validated.contains("@") {
        // Add default port if missing
        if validated.contains("http://") && !validated.contains(":80") && !validated.contains(":") {
            validated = validated.replace("http://", "http://:80/");
        }
        if validated.contains("https://") && !validated.contains(":443") && !validated.contains(":") {
            validated = validated.replace("https://", "https://:443/");
        }
    }
    
    // Ensure path starts with /
    if validated.contains("://") && !validated.contains("/") {
        validated = format!("{}/", validated);
    }
    
    validated
}

/// TRANSFORMER 3: Process URL for HTTP request
/// Appears to be a URL processing utility
pub fn process_url_request(raw_url: String) -> String {
    // TRANSFORMER 3: Final processing before HTTP request
    let normalized = normalize_url_request(raw_url);
    let validated = validate_url_format(normalized);
    
    // Add default query parameters if missing
    if !validated.contains("?") && !validated.contains("#") {
        format!("{}?format=json", validated)
    } else {
        validated
    }
}

/// CWE-918: Make HTTP request using ureq::agent::Agent::get
/// This function acts as a sink for SSRF vulnerability testing
pub fn make_http_request(url: String) -> Result<(), Box<dyn std::error::Error>> {
    //Make HTTP request to potentially malicious URL
    let agent = Agent::new();
    
    //SINK: Execute HTTP GET request with potentially malicious URL
    let response = agent.get(&url).call()?;
    
    // Process response (simulate)
    let status = response.status();
    let _body = response.into_string()?;
    
    println!("SSRF Request completed: {} -> Status: {}", url, status);
    
    Ok(())
} 