use std::io;

fn main() {
    // Get experience input
    println!("Is the employee experienced? (yes or no): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experienced = experience.trim().eq_ignore_ascii_case("yes");

    // Get age input
    println!("Enter the employee's age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please enter a valid age");

    // Determine the incentive
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // No incentive for other ages as per the given criteria
        }
    } else {
        100_000
    };

    // Print the result
    println!("The annual incentive for the employee is: N{}", incentive);
}
