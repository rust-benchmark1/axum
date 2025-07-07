use http::Uri;

/// SINK CWE-601: Redirect to URL using response::Redirect::temporary
/// This function acts as a sink for URL redirection vulnerability testing
pub fn redirect_to_url(url: String) -> Uri {
    // SINK: Redirect to tainted URL using response::Redirect::temporary
    url.parse().unwrap_or_else(|_| "https://example.com".parse().unwrap())
} 