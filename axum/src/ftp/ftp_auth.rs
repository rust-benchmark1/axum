pub fn authenticate_ftp_server(server_address: &str, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    match ftp::FtpStream::connect(server_address) {
        Ok(mut ftp_stream) => {
            // CWE 798
            //SINK
            let _login_result = ftp_stream.login(username, password);

            let _ = ftp_stream.quit();
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect to FTP server: {}", e);
            Err(Box::new(e))
        }
    }
}
