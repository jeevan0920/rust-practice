use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct MyMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

impl<T> MyMutex<T> {
    pub fn new(data: T) -> Self {
        MyMutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MyMutexGuard<T> {
        while self.locked.compare_and_swap(false, true, Ordering::Acquire) != false {}
        MyMutexGuard { mutex: self }
    }
}

pub struct MyMutexGuard<'a, T> {
    mutex: &'a MyMutex<T>,
}

impl<'a, T> Drop for MyMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> std::ops::Deref for MyMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for MyMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

use std::sync::Arc;
use std::thread;

fn main() {
    let mutex = Arc::new(MyMutex::new(0));

    let handles: Vec<_> = (0..1000)
        .map(|_| {
            let mutex = Arc::clone(&mutex);
            thread::spawn(move || {
                let mut guard = mutex.lock();
                *guard += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *mutex.lock());
}
