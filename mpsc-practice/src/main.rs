use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let mut threads = Vec::new();
    for i in 0..5 {
        let thread_tx = tx.clone(); // Clone the sender for each thread
        threads.push(thread::spawn(move || {
            let message = format!("message from thread {}", i);
            thread_tx
                .send(message)
                .expect(&format!("Error in sending message from thread {}", i));
        }));
    }

    drop(tx); // Drop the original sender so the receiver loop can exit

    for received in rx {
        println!("Message received at main thread: {:?}", received);
    }

    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}
