use std::io;

fn main() {
    // Prompt user to input the values of a, b, and c
    let a = input("Enter the value of a: ");
    let b = input("Enter the value of b: ");
    let c = input("Enter the value of c: ");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots based on the discriminant
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("The root is real and equal:");
        println!("Root = {}", root);
    } else {
        // No real roots
        println!("There are no real roots.");
    }
}

// Function to read input from the keyboard and parse it as a f64 (floating-point number)
fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    input_text.trim().parse().expect("Please enter a valid number")
}
