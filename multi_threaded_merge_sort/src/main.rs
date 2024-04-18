// Import necessary libraries
use rand::Rng;
use std::thread;
use std::time::Instant;

// Function to generate a random array of integers
fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..10000)).collect()
}

fn merge_sorted_arrays(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    if arrays.is_empty() {
        return Vec::new();
    } else if arrays.len() == 1 {
        return arrays[0].clone();
    }

    let mid_point = arrays.len() / 2;
    let left_half = merge_sorted_arrays(arrays[..mid_point].to_vec());
    let right_half = merge_sorted_arrays(arrays[mid_point..].to_vec());

    merge_two_sorted_arrays(left_half, right_half)
}

fn merge_two_sorted_arrays(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        merged.extend(&left[i..]);
    }
    if j < right.len() {
        merged.extend(&right[j..]);
    }

    merged
}

// Improved threading strategy
fn threaded_sort(data: Vec<i32>) -> Vec<i32> {
    let num_threads = num_cpus::get(); // Get the number of CPU cores
    println!("num_threads: {}", num_threads);
    let len = data.len();
    let chunk_size = (len + num_threads - 1) / num_threads; // Ensure even distribution

    let mut threads = vec![];

    for chunk in data.chunks(chunk_size) {
        let sub_array = chunk.to_vec();
        threads.push(thread::spawn(move || {
            println!("sub_array.len(): {}", sub_array.len());
            let mut sorted_sub_array = sub_array;
            let start = Instant::now();
            sorted_sub_array.sort();
            let duration = start.elapsed();
            println!("Sub array sort took: {:?}", duration);
            sorted_sub_array
        }));
    }

    let mut sorted_sub_arrays = Vec::with_capacity(threads.len());
    for thread in threads {
        sorted_sub_arrays.push(thread.join().unwrap());
    }

    merge_sorted_arrays(sorted_sub_arrays)
}
// Main function to run the program
fn main() {
    let size = 100000000;
    let data = generate_random_array(size);

    let start = Instant::now();
    let sorted_data = threaded_sort(data.clone());
    let duration = start.elapsed();
    println!("Multi-threaded sort took: {:?}", duration);

    // For comparison, perform a single-threaded sort
    let start_single = Instant::now();
    let mut single_sorted = data.clone();
    single_sorted.sort();
    let single_duration = start_single.elapsed();
    println!("Single-threaded sort took: {:?}", single_duration);

    // Output some of the sorted data to verify correctness
    println!("Sorted Data (multi-threaded): {:?}", &sorted_data[..10]);
    println!("Sorted Data (single-threaded): {:?}", &single_sorted[..10]);
}
