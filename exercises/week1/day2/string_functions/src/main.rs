use std::io;

/// Processes user input to create a greeting message.
/// Reads a name from stdin and passes it to a function to format a greeting.
fn main() {
    // Prompt user for name
    println!("Enter your name:");

    // Read input into a String.
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

    // Trim input and pass to greeting function
    let greeting = greet(input.trim());
    println!("{}", greeting);
}


/// Create a greeting message from a name.
/// # Arguments
/// * `name` - A string slice containing the name
/// # Returns
/// A formatted greeting as a String
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}