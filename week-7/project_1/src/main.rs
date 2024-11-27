use std::io;

fn main() {
    println!("Select the calculation you want to perform:");
    println!("1: Area of Trapezium");
    println!("2: Area of Rhombus");
    println!("3: Area of Parallelogram");
    println!("4: Area of Cube");
    println!("5: Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 5.");
            return;
        }
    };

    match choice {
        1 => area_of_trapezium(),
        2 => area_of_rhombus(),
        3 => area_of_parallelogram(),
        4 => area_of_cube(),
        5 => volume_of_cylinder(),
        _ => println!("Invalid choice. Please select a number between 1 and 5."),
    }
}

fn area_of_trapezium() {
    let (height, base1, base2) = get_three_inputs("height", "base1", "base2");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn area_of_rhombus() {
    let (diagonal1, diagonal2) = get_two_inputs("diagonal1", "diagonal2");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

fn area_of_parallelogram() {
    let (base, altitude) = get_two_inputs("base", "altitude");
    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn area_of_cube() {
    let side = get_single_input("length of the side");
    let area = 6.0 * side.powi(2);
    println!("The area of the cube is: {:.2}", area);
}

fn volume_of_cylinder() {
    let (radius, height) = get_two_inputs("radius", "height");
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn get_single_input(prompt: &str) -> f64 {
    println!("Enter the {}: ", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn get_two_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
    let val1 = get_single_input(prompt1);
    let val2 = get_single_input(prompt2);
    (val1, val2)
}

fn get_three_inputs(prompt1: &str, prompt2: &str, prompt3: &str) -> (f64, f64, f64) {
    let val1 = get_single_input(prompt1);
    let val2 = get_single_input(prompt2);
    let val3 = get_single_input(prompt3);
    (val1, val2, val3)
}
