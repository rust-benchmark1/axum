use std::net::TcpListener;
use std::io::Read;

pub fn return_items_list_html() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8099")?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = [0u8; 1024];
    // CWE 79
    //SOURCE
    let size = stream.read(&mut buffer)?;
    let items_data = std::str::from_utf8(&buffer[..size])?.to_string();

    let html_content = format!(
        "<html><body><h1>Items List</h1><ul><li>{}</li></ul></body></html>",
        items_data
    );

    // CWE 79
    //SINK
    let _response = actix_web::HttpResponse::Ok().body(html_content);

    Ok(())
}
