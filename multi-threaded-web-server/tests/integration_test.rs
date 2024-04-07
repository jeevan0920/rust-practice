use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;

#[test]
fn test_server_response() {
    // Assuming your server is running on localhost:7878
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // You can send a simple GET request
    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("response: {}", response);

    // Verify the response
    assert!(response.contains("HTTP/1.1 200 OK"));
}
