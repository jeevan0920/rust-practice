use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
// use std::os::unix::net::{UnixListener};
use tokio::net::{UnixStream, UnixListener};
use std::path::Path;
use std::thread;
use std::env;
use tokio_util::bytes::Bytes;
use tokio_util::codec::Framed;
use tokio_scgi::server::{SCGICodec, SCGIRequest};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client1(mut stream: UnixStream) -> io::Result<()> {
    // Buffer to store the received message
    let mut buffer = Vec::new();
    // Read the message from the client
    stream.read_to_end(&mut buffer);

    let mut framed: Framed<_, SCGICodec> = Framed::new(&mut stream, SCGICodec::new());

    // Print the received message
    let received_message = String::from_utf8_lossy(&buffer);
    println!("Received message: {}", received_message);

    // Sample response: Prefix "server response" to the request body
    let response = format!("server response{}", received_message);

    // Send the response back to the client
    // framed.send(Bytes::from(response)).await.unwrap();
    stream.write_all(response.as_bytes());

    Ok(())
}


// fn handle_client(mut stream: UnixStream) -> io::Result<()> {
//     // Buffer to store the received message
//     let mut buffer = Vec::new();
//     // Read the message from the client
//     stream.read_to_end(&mut buffer);

//     // Print the received message
//     let received_message = String::from_utf8_lossy(&buffer);
//     println!("Received message: {}", received_message);

//     // Sample response: Prefix "server response" to the request body
//     let response = format!("server response{}", received_message);

//     // Send the response back to the client
//     stream.write_all(response.as_bytes())?;

//     Ok(())
// }

fn main() -> io::Result<()> {
    // Define the path for the Unix domain socket
    let file_path = env::var("WSG_SOCKET_PATH").expect("WSG_SOCKET_PATH is not set");
    let socket_path = Path::new(&file_path);

    // Remove the existing socket file if it exists
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    // Create a Unix domain socket
    let listener = UnixListener::bind(socket_path)?;

    // Explicitly set permissions on the socket file
    std::fs::set_permissions(socket_path, std::fs::Permissions::from_mode(0o666))?;

    println!("Server listening on {:?}", socket_path);

    // Accept and handle incoming connections in a separate thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    if let Err(err) = handle_client1(stream).await {
                        eprintln!("Error handling client: {}", err);
                    }
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
                break;
            }
        }
    }

    Ok(())
}
