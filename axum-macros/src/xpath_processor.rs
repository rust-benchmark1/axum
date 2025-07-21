use std::io;

#[cfg(feature = "tokio")]
use tokio::net::UdpSocket;

use sxd_xpath::{Context, Factory, Value};

/// SOURCE CWE-643: Function to receive XPath query data from UDP socket
/// This function acts as a source for XPath injection vulnerability testing
#[cfg(feature = "tokio")]
pub async fn receive_xpath_query() -> io::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8084").await?;
    let mut buf = [0; 1024];
    // SOURCE: Receive data from UDP socket
    let len = socket.recv(&mut buf).await?;
    let query = String::from_utf8_lossy(&buf[..len]);
    Ok(query.to_string())
}

#[cfg(not(feature = "tokio"))]
pub async fn receive_xpath_query() -> io::Result<String> {
    // Fallback when tokio feature is not enabled
    Ok("//user[@id='1']".to_string())
}

/// TRANSFORMER 1: Normalize XPath query structure
/// Appears to be a legitimate XPath query normalization utility
pub fn normalize_xpath_query(raw_query: String) -> String {
    // TRANSFORMER 1: Normalize common XPath patterns and remove extra whitespace
    let mut normalized = raw_query.trim().to_string();
    
    // Replace common abbreviations
    normalized = normalized.replace("//", "/descendant::");
    normalized = normalized.replace("..", "parent::");
    normalized = normalized.replace(".", "self::");
    
    // Remove extra whitespace around operators
    normalized = normalized.replace(" = ", "=");
    normalized = normalized.replace(" != ", "!=");
    normalized = normalized.replace(" < ", "<");
    normalized = normalized.replace(" > ", ">");
    
    normalized
}

/// TRANSFORMER 2: Validate XPath query syntax
/// Appears to be a XPath query syntax validation utility
pub fn validate_xpath_syntax(query: String) -> String {
    // TRANSFORMER 2: Basic XPath syntax validation and cleanup
    let mut validated = query;
    
    // Ensure proper axis syntax
    if !validated.contains("::") && !validated.starts_with('/') && !validated.starts_with('.') {
        validated = format!("//{}", validated);
    }
    
    // Ensure proper attribute syntax
    if validated.contains("@") && !validated.contains("=") {
        validated = validated.replace("@", "@id='");
        validated.push_str("'");
    }
    
    validated
}

/// TRANSFORMER 3: Process XPath query for evaluation
/// Appears to be a XPath query processing utility
pub fn process_xpath_query(raw_query: String) -> String {
    // TRANSFORMER 3: Final processing before XPath evaluation
    let normalized = normalize_xpath_query(raw_query);
    let validated = validate_xpath_syntax(normalized);
    
    // Add default namespace if missing
    if !validated.contains("xmlns") && !validated.contains("namespace") {
        validated
    } else {
        validated
    }
}

/// SINK CWE-643: Evaluate XPath expression using sxd_xpath::XPath::evaluate
/// This function acts as a sink for XPath injection vulnerability testing
pub fn evaluate_xpath_expression(query: String) -> Result<(), Box<dyn std::error::Error>> {
    // SINK: Evaluate XPath expression using tainted query
    let factory = Factory::new();
    let xpath = factory.build(&query)
        .expect("Failed to build XPath outer")
        .expect("Failed to build XPath inner");
    let context = Context::new();
    // Load XML document from external file for evaluation
    let xml_content = std::fs::read_to_string("src/users.xml")?;
    let package = sxd_document::parser::parse(&xml_content)?;
    let document = package.as_document();
    // SINK: Execute XPath evaluation with potentially malicious query
    let _ = xpath.evaluate(&context, document.root())?;
    Ok(())
} 