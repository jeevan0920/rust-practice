// src/main.rs
extern "C" {
    fn square(x: i32) -> i32;
}

fn main() {
    let x = 4;
    let result: i32;
    unsafe {
        result = square(x);
    }
    println!("The square of {} is {}", x, result);
}
