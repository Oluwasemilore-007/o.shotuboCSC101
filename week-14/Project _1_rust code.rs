use std::env;
use postgres::{Client, NoTls, Error};

// Structs to hold column and table data (like containers)
struct Column {
    name: String,
    data_type: String,
}

struct Table {
    name: String,
    columns: Vec<Column>, // A table can have many columns
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get the role from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <role>"); // Tell the user how to use the program
    }

    let role = &args[1]; // The role is the second thing the user types

    // 2. Connect to the database (REPLACE WITH YOUR DETAILS)
    let mut client = Client::connect("host=localhost user=youruser password=yourpassword dbname=yourdb", NoTls)?;

    // 3. Do different things depending on the role
    let schema = match role.as_str() {
        "administrator" => {
            get_all_tables(&mut client)? // Get all tables for admin
        }
        "project_manager" => {
            get_one_table(&mut client, "project")? // Get only "project" table
        }
        "employee" => {
            get_one_table(&mut client, "staff")? // Get only "staff" table
        }
        "customer" => {
            get_one_table(&mut client, "customer")? // Get only "customer" table
        }
        "vendor" => {
            get_one_table(&mut client, "data_plan")? // Get only "data_plan" table
        }
        _ => panic!("Invalid role"), // Tell the user if they type the wrong role
    };

    // 4. Show the table information
    print_tables(&schema);

    Ok(())
}

// Function to get all tables and their columns
fn get_all_tables(client: &mut Client) -> Result<Vec<Table>, Error> {
    let mut all_tables = Vec::new(); // Make a list to hold all the tables

    // Get the names of all the tables
    let table_names = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[])?; // Adjust schema if needed

    // Loop through each table name
    for table_row in table_names {
        let table_name: String = table_row.get(0); // Get the table name from the row

        // Get the columns for the current table
        let columns = get_one_table(client, &table_name)?;

        // Add the table and its columns to the list
        all_tables.push(Table { name: table_name, columns });
    }

    Ok(all_tables)
}


// Function to get the columns for one table
fn get_one_table(client: &mut Client, table_name: &str) -> Result<Vec<Table>, Error> {
    let mut tables = Vec::new();
    let query = format!("SELECT table_name, column_name, data_type FROM information_schema.columns WHERE table_name = '{}' AND table_schema = 'public'", table_name);
    let rows = client.query(&query, &[])?;

    let mut current_table_name: Option<String> = None;
    let mut columns: Vec<Column> = Vec::new();

    for row in rows {
        let table_name: String = row.get(0);
        let column_name: String = row.get(1);
        let data_type: String = row.get(2);

        if current_table_name.is_none() || current_table_name.as_ref().unwrap() != &table_name {
            if let Some(name) = current_table_name {
                tables.push(Table { name, columns });
                columns = Vec::new();
            }
            current_table_name = Some(table_name);
        }

        columns.push(Column { name: column_name, data_type });
    }

    if let Some(name) = current_table_name {
        tables.push(Table { name, columns });
    }

    Ok(tables)
}

// Function to print the tables and columns nicely
fn print_tables(tables: &Vec<Table>) {
    for table in tables {
        println!("Table: {}", table.name);
        println!("--------------------");
        for column in &table.columns {
            println!("| {} | {}", column.name, column.data_type);
        }
        println!("--------------------");
        println!();
    }
}