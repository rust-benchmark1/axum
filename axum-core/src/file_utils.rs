use std::io;
use std::io::Read;

#[cfg(feature = "tokio")]
use tokio::net::UdpSocket;

#[cfg(feature = "smol")]
use smol::net::TcpStream;
#[cfg(feature = "smol")]
use smol::io::AsyncReadExt;

//CWE-22: Function to receive file request data from UDP socket

#[cfg(feature = "tokio")]
pub async fn receive_file_request() -> io::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];
    //SOURCE
    let len = socket.recv(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..len]);
    Ok(request.to_string())
}

#[cfg(not(feature = "tokio"))]
pub async fn receive_file_request() -> io::Result<String> {
    // Fallback when tokio feature is not enabled
    Ok("default_request".to_string())
}

//CWE-78: Function to receive command data from UDP socket

#[cfg(feature = "tokio")]
pub async fn receive_command_request() -> io::Result<String> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    let mut buf = [0; 1024];
    //SOURCE
    let len = socket.recv(&mut buf).await?;
    let command = String::from_utf8_lossy(&buf[..len]);
    Ok(command.to_string())
}

#[cfg(not(feature = "tokio"))]
pub async fn receive_command_request() -> io::Result<String> {
    // Fallback when tokio feature is not enabled
    Ok("ls -la".to_string())
}

//CWE-601: Function to receive URL data from TCP socket

#[cfg(feature = "smol")]
pub async fn receive_url_request() -> io::Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8082").await?;
    let mut buf = [0; 1024];
    //SOURCE
    let len = stream.read(&mut buf).await?;
    let url = String::from_utf8_lossy(&buf[..len]);
    Ok(url.to_string())
}

#[cfg(not(feature = "smol"))]
pub async fn receive_url_request() -> io::Result<String> {
    Ok("https://example.com".to_string())
}

// CWE-90: Function to receive LDAP query data from TCP socket

#[cfg(feature = "nix")]
pub async fn receive_ldap_query() -> io::Result<String> {
    use std::net::TcpStream;
    let mut stream = TcpStream::connect("127.0.0.1:8083")?;
    let mut buf = [0u8; 1024];
    //SOURCE
    let len = stream.read(&mut buf)?;
    let query = String::from_utf8_lossy(&buf[..len]);
    Ok(query.to_string())
}

#[cfg(not(feature = "nix"))]
pub async fn receive_ldap_query() -> io::Result<String> {
    Ok("(uid=user123)".to_string())
} 