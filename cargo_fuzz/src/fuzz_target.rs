// src/fuzz_target.rs
#![no_main]

use fuzz_testing::add_numbers;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Use the input data to generate two numbers
    if data.len() < 8 {
        return;
    }

    let a = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let b = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);

    // Call the function with the generated numbers
    let _result = add_numbers(a, b);
});
