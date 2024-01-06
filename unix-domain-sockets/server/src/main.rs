use std::io::{self, Read};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;

fn handle_client(mut stream: UnixStream) -> io::Result<()> {
    // Buffer to store the received message
    let mut buffer = [0; 128];
    // Read the message from the client
    let size = stream.read(&mut buffer)?;

    // Print the received message
    let received_message = String::from_utf8_lossy(&buffer[..size]);
    println!("Received message: {}", received_message);

    Ok(())
}

fn main() -> io::Result<()> {
    // Define the path for the Unix domain socket
    let socket_path = Path::new("   ");

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
                    if let Err(err) = handle_client(stream) {
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
