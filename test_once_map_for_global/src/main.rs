use std::collections::HashMap;
use once_cell::sync::OnceCell;

// Define the global hashmap wrapped in a OnceCell
static GLOBAL_HASHMAP: OnceCell<HashMap<i32, String>> = OnceCell::new();

fn initialize_global_hashmap() -> &'static HashMap<i32, String> {
    // Function to initialize the global hashmap (to be called only once)
    // In this example, we'll initialize it with some key-value pairs
    let mut map = HashMap::new();
    map.insert(1, "Value 1".to_string());
    map.insert(2, "Value 2".to_string());
    // Add more key-value pairs as needed

    // Store the hashmap in the OnceCell
    GLOBAL_HASHMAP.get_or_init(|| map)
}

fn main() {
    // Initialize the global hashmap
    let global_map = initialize_global_hashmap();

    // Now you can spawn threads to serve requests that read from the hashmap
    let thread_count = 5;
    let mut handles = vec![];

    for i in 0..thread_count {
        let handle = std::thread::spawn(move || {
            // Read from the global hashmap
            if let Some(value) = global_map.get(&1) {
                println!("Thread {}: {}", i, value);
            } else {
                println!("Thread {}: Key not found", i);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
    