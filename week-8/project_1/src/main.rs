use std::io;

fn main() {
    // Step 1: Define data using vectors
    let office_administrator = vec![
        ("Intern", "APS 1-2"),
        ("Administrator", "APS 3-5"),
        ("Senior Administrator", "APS 5-8"),
        ("Office Manager", "EL 8-10"),
        ("Director", "EL 10-13"),
        ("CEO", "SES"),
    ];

    let lawyer = vec![
        ("Paralegal", "APS 1-2"),
        ("Junior Associate", "APS 3-5"),
        ("Associate", "APS 5-8"),
        ("Senior Associate 1-2", "EL 8-10"),
        ("Senior Associate 3-4", "EL 10-13"),
        ("Partner", "SES"),
    ];

    let teacher = vec![
        ("Placement", "APS 1-2"),
        ("Classroom Teacher", "APS 3-5"),
        ("Snr Teacher", "APS 5-8"),
        ("Leading Teacher", "EL 8-10"),
        ("Deputy Principal", "EL 10-13"),
        ("Principal", "SES"),
    ];

    // Step 2: Get user input
    let role = get_input("Enter the role (Office Administrator, Lawyer, Teacher): ");
    let position = get_input("Enter the position: ");
    let experience: u32 = get_input("Enter years of experience: ")
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Step 3: Determine the APS level
    let aps_level = if role == "Office Administrator" {
        find_aps_level(&office_administrator, &position)
    } else if role == "Lawyer" {
        find_aps_level(&lawyer, &position)
    } else if role == "Teacher" {
        find_aps_level(&teacher, &position)
    } else {
        None
    };

    // Step 4: Display result
    match aps_level {
        Some(level) => {
            if validate_experience(level, experience) {
                println!("The APS level is: {}", level);
            } else {
                println!("The experience does not match the APS level.");
            }
        }
        None => println!("Invalid role or position."),
    }
}

// Step 5: Function to find APS level for a given role and position
fn find_aps_level<'a>(data: &'a Vec<(&'a str, &'a str)>, position: &'a str) -> Option<&'a str> {
    for &(pos, level) in data {
        if pos == position {
            return Some(level);
        }
    }
    None
}

// Step 6: Function to validate years of experience
fn validate_experience(level: &str, experience: u32) -> bool {
    match level {
        "APS 1-2" => experience >= 1 && experience <= 2,
        "APS 3-5" => experience >= 3 && experience <= 5,
        "APS 5-8" => experience >= 5 && experience <= 8,
        "EL 8-10" => experience >= 8 && experience <= 10,
        "EL 10-13" => experience >= 10 && experience <= 13,
        "SES" => experience > 13,
        _ => false,
    }
}

// Helper function to get user input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

