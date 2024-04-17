use std::collections::HashMap;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

static GLOBAL_MAP: AtomicPtr<HashMap<String, String>> = AtomicPtr::new(ptr::null_mut());

fn main() {
    let initial_map = HashMap::from([("key1".to_string(), "value1".to_string())]);
    update_global_map(initial_map);

    let handles: Vec<_> = (0..5)
        .map(|_| {
            std::thread::spawn(|| {
                let map = read_global_map();
                if let Some(value) = map.get("key1") {
                    println!("Thread reads: {}", value);
                }
            })
        })
        .collect();

    let new_map = HashMap::from([
        ("key1".to_string(), "new_value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]);
    update_global_map(new_map);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn update_global_map(new_map: HashMap<String, String>) {
    let new_map = Arc::new(new_map);
    let new_ptr = Arc::into_raw(new_map) as *mut HashMap<String, String>;
    let old_ptr = GLOBAL_MAP.swap(new_ptr, Ordering::SeqCst);
    if !old_ptr.is_null() {
        unsafe {
            Arc::from_raw(old_ptr);
        }
    }
}

fn read_global_map() -> Arc<HashMap<String, String>> {
    let ptr = GLOBAL_MAP.load(Ordering::SeqCst);
    if ptr.is_null() {
        panic!("Global map is not initialized.");
    } else {
        unsafe { Arc::clone(&*ptr) }
    }
}
