/// Demonstrate basic Rust functions for arithmetic operations.
/// Defines and calls functions for addition and multiplication.
fn main() {
    let sum = add(3, 8);
    let product = multiply(3, 8);
    println!("Sum: {}", sum);
    println!("Product: {}", product);
}

/// Adds two integers and returns their sum.
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
/// # Returns
/// Sum of `a` and `b`
fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return
}

/// Multiplies two integers and return their product.
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
/// # Returns
/// Product of `a` and `b`
fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // Explicit return
}

