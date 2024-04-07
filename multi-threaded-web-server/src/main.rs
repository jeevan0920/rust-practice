use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::spawn,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let request_count = Arc::new(Mutex::new(0));

    let (log_sender, log_receiver) = mpsc::channel::<String>();
    let log_sender = Arc::new(log_sender);

    let logger_handle = {
        spawn(move || {
            for log in log_receiver {
                println!("LOG: {}", log);
            }
        })
    };

    for stream in listener.incoming() {
        let count = request_count.clone();
        let log_sender = log_sender.clone();

        let stream = stream.unwrap();

        spawn(move || {
            let mut num = count.lock().unwrap();
            *num += 1;
            let message = format!(
                "Handling connection {} request count {}",
                stream.peer_addr().unwrap(),
                *num
            );
            log_sender.send(message).unwrap();
            handle_connection(stream);
        });
    }

    println!("Before joining the logger_handle");
    logger_handle.join().unwrap();
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
