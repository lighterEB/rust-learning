/// Calculates the area of a circle base on user-input radius.
/// Handles invalid inputs(non-numeric or negative values) and prints the result
/// formatted to two decimal places.
/// Author: LighterEB
/// Created: 2025-07-10
/// Purpose: understand variable statement

use std::io;
fn main() {
    // Constant for pi
    const PI: f64 = 3.14159265359;
    // Prompt user for radius input
    println!("Enter the radius of the circle:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let radius: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid number");
            return;
        }
    };

    if radius < 0.0 {
        println!("Radius cannot be negative");
        return;
    }

    // calculate area: π * r^2
    let area: f64 = PI * radius * radius;
    // formatted to two decimal places
    println!("Area of the circle: {:.2} square units", area);
}
