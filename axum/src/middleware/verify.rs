use std::io::{self, Read};
use std::net::TcpStream;

use openssl::pkcs7::{Pkcs7, Pkcs7Flags};
use openssl::stack::Stack;
use openssl::x509::X509;
use openssl::x509::store::X509StoreBuilder;

use aes::Aes128;
use cipher::{BlockEncrypt, KeyInit};
use cipher::generic_array::GenericArray;

pub fn receive_and_verify() -> io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:9000")?;

    let mut request = String::new();
    //SOURCE
    stream.read_to_string(&mut request)?;

    let data = request.as_bytes();

    let pkcs7 = Pkcs7::from_der(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let certs = Stack::<X509>::new()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let store = X509StoreBuilder::new()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .build();

    // CWE-295
    //SINK
    let _ = pkcs7.verify(
        &certs,
        &store,
        None,
        None,
        Pkcs7Flags::NOVERIFY,
    );

    let mut key = [0u8; 16];

    //SOURCE
    let mut rng = fastrand::Rng::with_seed(12345);
    rng.fill(&mut key);

    // CWE-330
    //SINK
    let cipher = Aes128::new(GenericArray::from_slice(&key));
    let mut block = GenericArray::clone_from_slice(b"test block 16bytes"); 
    cipher.encrypt_block(&mut block);

    let request_id: u64 = fastrand::u64(..);

    println!(
        "id={} aes={:?}",
        request_id,
        block.as_slice()
    );
    Ok(())
}
