use std::env;

use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use bytes::{Bytes, BytesMut, BufMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio_scgi::client::SCGIRequest;
// use tokio_scgi::client::ScgiBuilder;
// use futures_util::stream::stream::StreamExt;

use tokio_scgi::client::{SCGICodec};
use tokio_util::codec::Framed;
use futures_util::SinkExt;
use futures_util::StreamExt;

// #[tokio::main]
// async fn main2() -> Result<(), io::Error> {
//     // Unix domain socket path
//     let socket_path = "/path/to/unix/socket.sock";

//     // Create an HTTP request (modify as needed)
//     let http_request = "POST /path HTTP/1.1\r\nHost: localhost\r\nContent-Length: 12\r\n\r\nHello, world!";

//     // Convert HTTP request to SCGI format (modify as needed)
//     let scgi_request = http_to_scgi(http_request);

//     // Connect to the Unix domain socket
//     let stream = UnixStream::connect(socket_path).await?;

//     // Create a Framed instance with SCGI codec
//     let mut framed = Framed::new(stream, SCGICodec::new());

//     // Send the SCGI request
//     framed.send(scgi_request).await?;

//     // Read the response (modify as needed)
//     while let Some(response) = framed.next().await {
//         println!("Received SCGI Response: {:?}", response?);
//     }

//     Ok(())
// }

// pub async fn hb_handler(body: Bytes, data: HttpRequest) -> impl Responder {
//     println!("{:#?}",body);
//     COUNT_HBS.inc();
//     send(body);
//     HttpResponse::Ok().content_type(ContentType::plaintext()).body("TBD")
//   }

async fn handle_request(http_request: HttpRequest, body: Bytes) -> impl Responder {
        // Convert HTTP request to SCGI format (modify as needed)
        let scgi_request = http_to_scgi(&http_request, &body).await;

        let socket_path = env::var("WSG_SOCKET_PATH").unwrap();

        // Connect to the Unix domain socket
        let stream = UnixStream::connect(socket_path).await.unwrap();
    
        // Create a Framed instance with SCGI codec
        let mut framed = Framed::new(stream, SCGICodec::new());
    
        // Send the SCGI request
        framed.send(scgi_request).await.unwrap();
    
        // Read the response (modify as needed)
        while let Some(response) = framed.next().await {
            println!("Received SCGI Response: {:?}", response.unwrap());
        }    

        HttpResponse::Ok().content_type(ContentType::plaintext()).body("TBD")
}

// impl Responder {
//     println!("{:#?}",body);
//     COUNT_HBS.inc();
//     send(body);
//     HttpResponse::Ok().content_type(ContentType::plaintext()).body("TBD")
//   }

// Function to convert actix-web HttpRequest to SCGI format
async fn http_to_scgi(req: &HttpRequest, body: &[u8]) -> SCGIRequest {
    // Extract relevant information from the actix-web HttpRequest and create an SCGI request
    // You need to handle headers, body, and any other necessary details here.
    // This is a simplified example and might not cover all cases.

    // Extract headers
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(name, value)| (name.as_str().to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    // Extract body
    // let mut body = web::Bytes::from(req.body().await.unwrap().to_vec());

    SCGIRequest::Request(headers, BytesMut::from(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::post().to(handle_request)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
