use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct SafeQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> SafeQueue<T> {
    fn new() -> Self {
        SafeQueue {
            queue: Mutex::new(VecDeque::<T>::new()),
            condvar: Condvar::new(),
        }
    }

    fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        self.condvar.notify_one()
    }

    fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        queue.pop_front()
    }
}

fn main() {
    let queue = Arc::new(SafeQueue::<i32>::new());

    let mut threads = Vec::new();

    // producer threads which are trying to push to queue
    for i in 0..5000 {
        let queue = queue.clone();
        threads.push(thread::spawn(move || {
            queue.enqueue(i);
        }));
    }

    // consumer threads which are trying to read from the queue
    for i in 0..50 {
        let queue = queue.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..100 {
                println!(
                    "reading from consumer thread: {} and data is {:?}",
                    i,
                    queue.dequeue()
                )
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
