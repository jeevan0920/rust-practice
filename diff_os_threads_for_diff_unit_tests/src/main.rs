use std::thread::{self, ThreadId};

// Function to get the current thread ID
fn get_thread_id() -> ThreadId {
    thread::current().id()
}

// Function to print the environment variables
fn print_environment() {
    println!("Thread ID: {:?}", get_thread_id());

    // Iterate over the environment variables and print them
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    println!();
}

#[test]
fn test_one() {
    // Print thread ID and environment variables for the first test
    println!("Test One:");
    print_environment();

    // Your test code goes here

    // Asserts or other test logic goes here
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_two() {
    // Print thread ID and environment variables for the second test
    println!("Test Two:");
    print_environment();

    // Your test code goes here

    // Asserts or other test logic goes here
    assert_eq!(3 * 3, 9);
}

fn main() {
    // Run the tests
    println!("Running tests...");

    // // Run each test in a separate thread
    // let thread_one = thread::spawn(|| {
    //     test_one();
    // });

    // let thread_two = thread::spawn(|| {
    //     test_two();
    // });

    // // Wait for the threads to finish
    // thread_one.join().unwrap();
    // thread_two.join().unwrap();

    println!("Tests completed.");
}
