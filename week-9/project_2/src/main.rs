use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: Define student details as a vector of tuples
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 100),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    // Step 2: Print the student details to the console
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{:-<20} {:-<15} {:-<15} {:-<5}", "", "", "", "");

    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.0, student.1, student.2, student.3
        );
    }

    // Step 3: Prepare the data to write to a file
    let mut file_content = String::new();
    file_content.push_str("Student Name,Matric. Number,Department,Level\n");
    for student in &students {
        file_content.push_str(&format!("{},{},{},{}\n", student.0, student.1, student.2, student.3));
    }

    // Step 4: Write the data to a CSV file
    let file_name = "student_details.csv";
    let mut file = File::create(file_name).expect("Could not create file");
    file.write_all(file_content.as_bytes())
        .expect("Could not write to file");

    // Step 5: Notify the user
    println!("\nStudent details have been saved to '{}'.", file_name);
}
