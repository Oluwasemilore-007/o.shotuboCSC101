use std::io;

fn main() {
    println!("Welcome to the EY Candidate Evaluator!");
    println!("Enter each candidate's name and years of experience. You must enter at least two candidates. Type 'done' to finish after that.\n");

    let mut candidates = Vec::new();
    let mut first_entry = true;

    loop {
        if first_entry {
            println!("Enter a candidate's name:");
            first_entry = false;
        } else {
            println!("Enter another candidate's name:");
        }

        // Get candidate's name
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        // Check if user tries to type "done" before adding at least two candidates
        if name.eq_ignore_ascii_case("done") && candidates.len() < 2 {
            println!("You must enter at least two candidates before typing 'done'.");
            continue;
        }

        if name.eq_ignore_ascii_case("done") {
            break;
        }

        // Get candidate's years of experience
        println!("Enter the number of years of experience {} has had:", name);
        let mut experience_input = String::new();
        io::stdin().read_line(&mut experience_input).unwrap();
        let experience: u32 = match experience_input.trim().parse() {
            Ok(exp) => exp,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                continue;
            }
        };

        // Add the candidate to the list
        candidates.push((name.to_string(), experience));

        // After the second candidate, display "Type 'done' to end"
        if candidates.len() == 2 {
            println!("You can now type 'done' to end if there are no more candidates or proceed to add more candidates.");
        }
    }

    // Find the candidate with the highest experience
    let mut top_candidate = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > top_candidate.1 {
            top_candidate = candidate;
        }
    }

    // Display the result
    println!(
        "\nThe candidate with the highest experience is {} with {} years of experience.",
        top_candidate.0, top_candidate.1
    );
}
