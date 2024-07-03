use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// Define a type for Events
type Event = String;

// EventBus structure
struct EventBus {
    subscribers: Vec<Sender<Event>>,
    queue: Arc<(Mutex<VecDeque<Event>>, Condvar)>,
}

impl EventBus {
    // Create a new EventBus
    fn new() -> Self {
        EventBus {
            subscribers: Vec::new(),
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    // Subscribe to the event bus
    fn subscribe(&mut self) -> Receiver<Event> {
        let (tx, rx) = channel();
        self.subscribers.push(tx);
        rx
    }

    // Publish an event to the bus
    fn publish(&self, event: Event) {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        queue.push_back(event);
        cvar.notify_one();
    }

    // Start the event processing threads
    fn start(&self, num_threads: usize) {
        let queue = Arc::clone(&self.queue);
        let subscribers = self.subscribers.clone();

        for _ in 0..num_threads {
            let queue = Arc::clone(&queue);
            let subscribers = subscribers.clone();

            thread::spawn(move || {
                loop {
                    let event;
                    {
                        let (lock, cvar) = &*queue;
                        let mut queue = lock.lock().unwrap();
                        while queue.is_empty() {
                            queue = cvar.wait(queue).unwrap();
                        }
                        event = queue.pop_front().unwrap();
                    }

                    // Dispatch event to all subscribers
                    for subscriber in &subscribers {
                        if let Err(_) = subscriber.send(event.clone()) {
                            // Handle the case where the subscriber might have disconnected
                        }
                    }
                }
            });
        }
    }
}

fn main() {
    let mut bus = EventBus::new();

    // Subscribe to the bus
    let rx1 = bus.subscribe();
    let rx2 = bus.subscribe();

    // Start the event bus with 2 worker threads
    bus.start(2);

    // Spawn a thread to listen for events
    let listener1 = thread::spawn(move || {
        while let Ok(event) = rx1.recv() {
            println!("Listener 1 received: {}", event);
        }
    });

    // Spawn another thread to listen for events
    let listener2 = thread::spawn(move || {
        while let Ok(event) = rx2.recv() {
            println!("Listener 2 received: {}", event);
        }
    });

    // Publish some events
    bus.publish("Event 1".to_string());
    bus.publish("Event 2".to_string());
    bus.publish("Event 3".to_string());

    // Wait for listeners to finish
    listener1.join().unwrap();
    listener2.join().unwrap();
}
