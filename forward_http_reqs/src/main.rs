use actix_web::web::Bytes;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use actix_web::HttpRequest;
use std::io::{Write, Read};
// use std::io::{Write, Read, Bytes};
use std::os::unix::net::UnixStream;
use std::sync::Mutex;


#[derive(Debug, Deserialize)]
struct MyData {
    // Define your data structure here
    // Example: field1: String, field2: i32
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SOCKET: Mutex<UnixStream>  = Mutex::new(UnixStream::connect("/tmp/src").unwrap());
}

fn main1() {
    SOCKET.lock().unwrap().write_all(b"hello world").unwrap();
    let mut response = String::new();
    SOCKET.lock().unwrap().read_to_string(&mut response).unwrap();
    println!("{response}");
}

#[post("/my-endpoint")]
async fn my_handler(data: Bytes, req:HttpRequest) -> HttpResponse {
    // Access the request headers
    let headers = req.headers();

    SOCKET.lock().unwrap().write_all(&data).unwrap();


    // Process the extracted data and headers as needed
    // Example: println!("Received data: {:?}", data);
    // Example: println!("Headers: {:?}", headers);

    // Return a response
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(my_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
