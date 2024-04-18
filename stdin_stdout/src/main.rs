use std::io::{self, stdin};
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    println!("Enter integers separated by spaces: ");
    let mut input = String::new();

    // read input from stdin
    stdin().read_line(&mut input).expect("failed to read input");

    // parse the numbers from the input
    numbers = input
        .split_whitespace()
        .filter_map(|num| i32::from_str(num).ok())
        .collect();

    // print the numbers
    println!("numbers: {:?}", numbers);
}
