fn main() {
    // Dataset 1: Names of commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Combine the datasets into a single vector
    let mut merged_data = Vec::new();
    for i in 0..commissioners.len() {
        merged_data.push((
            i + 1, // Serial Number (S/N)
            commissioners[i],
            ministries[i],
            zones[i],
        ));
    }

    // Display the merged data in a table format
    println!(
        "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone"
    );
    println!("{:-<5} {:-<30} {:-<20} {:-<15}", "", "", "", "");

    for entry in &merged_data {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            entry.0, entry.1, entry.2, entry.3
        );
    }

    // Save the merged data to a file
    let file_name = "merged_datasets.csv";
    let mut file_content = String::new();
    file_content.push_str("S/N,Name of Commissioner,Ministry,Geopolitical Zone\n");
    for entry in &merged_data {
        file_content.push_str(&format!("{},{},{},{}\n", entry.0, entry.1, entry.2, entry.3));
    }

    std::fs::write(file_name, file_content).expect("Unable to write file");
    println!("\nData has been saved to '{}'.", file_name);
}
