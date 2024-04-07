#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let input = String::from_utf8_lossy(data);
    println!("Input: {}", input);

    // Example function to fuzz:
    // Replace this with the function you want to test
    let result = reverse_string(&input);

    println!("Reversed: {}", result);
});

fn reverse_string(input: &str) -> String {
    // Example function with a potential vulnerability
    input.chars().rev().collect()
}
