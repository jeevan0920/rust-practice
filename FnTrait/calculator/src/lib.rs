pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}

pub fn mul(left: i32, right: i32) -> i32 {
    left * right
}

pub fn div(left: i32, right: i32) -> i32 {
    left / right
}

pub fn calculator_with_fn_ptrs(left: i32, right: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(left, right)
}

pub fn calculator_with_fn_trait_objects<F>(left: i32, right: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_pointers() {
        let a = 10;
        let b = 5;

        println!("Addition: {}", calculator_with_fn_ptrs(a, b, add));
        println!("Subtraction: {}", calculator_with_fn_ptrs(a, b, sub));
        println!("Multiplication: {}", calculator_with_fn_ptrs(a, b, mul));
        println!("Division: {}", calculator_with_fn_ptrs(a, b, div));
    }

    #[test]
    fn test_fn_trait_objects() {
        let a = 10;
        let b = 5;

        println!("Addition: {}", calculator_with_fn_trait_objects(a, b, add));
        println!(
            "Subtraction: {}",
            calculator_with_fn_trait_objects(a, b, sub)
        );
        println!(
            "Multiplication: {}",
            calculator_with_fn_trait_objects(a, b, mul)
        );
        println!("Division: {}", calculator_with_fn_trait_objects(a, b, div));

        // Using closures
        let add_closure = |x, y| x + y;
        println!(
            "Addition (closure): {}",
            calculator_with_fn_trait_objects(a, b, add_closure)
        );
    }
}
