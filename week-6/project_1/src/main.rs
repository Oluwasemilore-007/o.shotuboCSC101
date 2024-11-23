use std::io;

fn main() {
    // Display the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken       - N3,000");
    println!("A = Amala & Ewedu Soup         - N2,500");
    println!("E = Eba & Egusi Soup           - N2,000");
    println!("W = White Rice & Stew          - N2,500");

    // Variables for total cost
    let mut total = 0;

    loop {
        println!("\nEnter the food code (or type 'X' to finish):");

        // Read the food code
        let mut food_code = String::new();
        io::stdin()
            .read_line(&mut food_code)
            .expect("Failed to read input");
        let food_code = food_code.trim().to_uppercase();

        if food_code == "X" {
            break; // Exit the loop when the user types 'X'
        }

        // Determine the price of the selected food
        let price = match food_code.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid food code! Please try again.");
                continue;
            }
        };

        println!("Enter the quantity:");

        // Read the quantity
        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read input");

        // Convert quantity to an integer
        let quantity: i32 = match quantity.trim().parse() {
            Ok(q) => q,
            Err(_) => {
                println!("Invalid quantity! Please enter a number.");
                continue;
            }
        };

        // Add the cost of this item to the total
        total += price * quantity;
        println!("Item added! Subtotal: N{}", total);
    }

    // Check if a discount applies
    if total > 10000 {
        let discount = total as f64 * 0.05;
        total = (total as f64 - discount) as i32;
        println!("\nA 5% discount has been applied!");
    }

    // Display the total cost
    println!("\nTotal bill: N{}", total);
    println!("Thank you for your order!");
}
