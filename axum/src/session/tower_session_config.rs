use tower_sessions::{SessionManagerLayer, MemoryStore};

pub fn configure_session_layer(auth_token: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let session_data = format!("auth_token={}; user_id={}", auth_token, user_id);

    let session_store = MemoryStore::default();

    let _session_layer = SessionManagerLayer::new(session_store)
        // CWE 1004
        //SINK
        .with_http_only(false)
        // CWE 614
        //SINK
        .with_secure(false);

    Ok(())
}
