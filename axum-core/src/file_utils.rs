use std::io;

#[cfg(feature = "tokio")]
use tokio::net::UdpSocket;

/// SOURCE CWE-22: Function to receive file request data from UDP socket
/// This function acts as a source for path traversal vulnerability testing
#[cfg(feature = "tokio")]
pub async fn receive_file_request() -> io::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];
    // SOURCE: Receive data from UDP socket
    let len = socket.recv(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..len]);
    Ok(request.to_string())
}

#[cfg(not(feature = "tokio"))]
pub async fn receive_file_request() -> io::Result<String> {
    // Fallback when tokio feature is not enabled
    Ok("default_request".to_string())
}

/// SOURCE CWE-78: Function to receive command data from UDP socket
/// This function acts as a source for OS command injection vulnerability testing
#[cfg(feature = "tokio")]
pub async fn receive_command_request() -> io::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    let mut buf = [0; 1024];
    // SOURCE: Receive command data from UDP socket
    let len = socket.recv(&mut buf).await?;
    let command = String::from_utf8_lossy(&buf[..len]);
    Ok(command.to_string())
}

#[cfg(not(feature = "tokio"))]
pub async fn receive_command_request() -> io::Result<String> {
    // Fallback when tokio feature is not enabled
    Ok("ls -la".to_string())
} 