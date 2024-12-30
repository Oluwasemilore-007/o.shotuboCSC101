use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: List the drinks under each category
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Step 2: Prepare the data as a simple text format
    let mut file_content = String::new();

    // Add Lager drinks
    file_content.push_str("Lager:\n");
    for drink in lager.iter() {
        file_content.push_str(&format!(" - {}\n", drink));
    }

    // Add a space between categories
    file_content.push_str("\n");

    // Add Stout drinks
    file_content.push_str("Stout:\n");
    for drink in stout.iter() {
        file_content.push_str(&format!(" - {}\n", drink));
    }

    // Add a space between categories
    file_content.push_str("\n");

    // Add Non-Alcoholic drinks
    file_content.push_str("Non-Alcoholic:\n");
    for drink in non_alcoholic.iter() {
        file_content.push_str(&format!(" - {}\n", drink));
    }

    // Step 3: Write the data to a file
    let file_name = "drinks.txt";
    let mut file = File::create(file_name).expect("Could not create file");
    file.write_all(file_content.as_bytes()).expect("Could not write to file");

    // Step 4: Confirm that the file was created
    println!("The drinks data has been saved to '{}'.", file_name);
}
