use crate::response::Redirect;
use axum_core::file_utils::receive_url_request;

//CWE-601: Redirect to URL using response::Redirect::to
pub async fn redirect_url_user() -> Redirect {
    let tainted_url = receive_url_request().await.unwrap_or_else(|_| "https://example.com".to_string());
    //SINK
    Redirect::to(&tainted_url)
} 