use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;
use std::thread;

fn main() -> io::Result<()> {
    // Define the path for the Unix domain socket
    let socket_path = Path::new("/tmp/my_unix_socket");

    // Introduce a delay before attempting to connect
    thread::sleep(Duration::from_secs(1));

    // Create a Unix domain socket
    let mut stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("Error in connecting to socket path {:?}: {}", socket_path, err);
            return Err(err);
        }
    };

    // Message to be sent
    let message = "Hello, Unix domain socket!";

    // Write the message to the socket
    stream.write_all(message.as_bytes())?;

    println!("Message sent to Unix domain socket: {}", message);

    Ok(())
}
