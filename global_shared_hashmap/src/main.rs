use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

// Define a global static hashmap using lazy_static
lazy_static! {
    static ref HASHMAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

fn main() {
    // Spawn a thread to update the hashmap every 10 seconds
    thread::spawn(|| loop {
        match reload_hashmap_from_file("data.tex") {
            Ok(_) => println!("Hashmap reloaded successfully."),
            Err(e) => eprintln!("Failed to reload hashmap: {}", e),
        }
        thread::sleep(Duration::from_secs(10));
    });

    // Example of accessing the hashmap from another thread
    thread::spawn(|| loop {
        {
            let hashmap = HASHMAP.read().unwrap(); // Safely read the hashmap
            for (key, value) in hashmap.iter() {
                println!("{} -> {}", key, value);
            }
        }
        thread::sleep(Duration::from_secs(1));
    });

    // Keep the main thread alive indefinitely
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn reload_hashmap_from_file(filename: &str) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut new_hashmap = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            new_hashmap.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    let mut hashmap = HASHMAP.write().unwrap();
    *hashmap = new_hashmap;
    Ok(())
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::io::Write;
    use std::sync::{Arc, Barrier};
    use tempfile::NamedTempFile;

    fn setup_dummy_file(contents: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", contents).unwrap();
        file
    }

    #[test]
    fn test_concurrent_read_and_write() {
        let file = setup_dummy_file("key1->value1\nkey2->value2");
        let filename = file.path().to_str().unwrap().to_string();

        let barrier = Arc::new(Barrier::new(11)); // 10 readers + 1 writer

        // Spawn writer thread
        let writer_handle = {
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait(); // Synchronize the start of all threads
                reload_hashmap_from_file(&filename).unwrap();
            })
        };

        // Spawn reader threads
        let reader_handles: Vec<_> = (0..10)
            .map(|_| {
                let barrier = barrier.clone();
                thread::spawn(move || {
                    barrier.wait(); // Ensure all readers start simultaneously with the writer
                    thread::sleep(Duration::from_millis(100)); // Wait to simulate read during write operation
                    let hashmap = HASHMAP.read().unwrap();
                    (hashmap.get("key1").cloned(), hashmap.get("key2").cloned())
                })
            })
            .collect();

        // Wait for writer to complete
        writer_handle.join().unwrap();

        // Check reader outputs
        for handle in reader_handles {
            let (key1, key2) = handle.join().unwrap();
            assert_eq!(key1, Some("value1".to_string()));
            assert_eq!(key2, Some("value2".to_string()));
        }
    }
}
