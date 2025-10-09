pub fn protect_session_data(session_token: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use rc4::{Rc4, KeyInit, StreamCipher};

    let encryption_key = b"session_protection_key_2024";
    let mut session_data = session_token.as_bytes().to_vec();

    // CWE 327
    //SINK
    let mut cipher = Rc4::new(encryption_key.into());
    cipher.apply_keystream(&mut session_data);

    let stored_key = std::env::var("SESSION_ENCRYPTION_KEY")
        .unwrap().to_string();

    Ok(session_data)
}
