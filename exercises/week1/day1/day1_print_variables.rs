fn main() {
    let age: i32 = 18;
    let mut height: f64 = 1.78;
    let name: &str = "LighterEB";
    let is_student: bool = true;
    let initial: char = 'L';

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} meters", height);
    println!("Is student: {}", is_student);
    println!("Initial: {}", initial);

    height = 1.80;
    println!("Update height: {} meters", height);
    
}