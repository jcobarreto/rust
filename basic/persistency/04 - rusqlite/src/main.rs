mod models;
mod screen;
mod config;
mod repositories;

use std::io;
use screen::display;

fn main() {
    loop {
        println!("CRUD Clients");
        println!("1. Create Client");
        println!("2. List Clients");
        println!("3. Update Client");
        println!("4. Delete Client");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let result = match choice.trim() {
            "1" => display::create_client(),
            "2" => display::list_clients(),
            "3" => display::update_client(),
            "4" => display::delete_client(),
            "5" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid option, please try again.");
                Ok(()) // Returns Ok to maintain type consistency
            },
        };

        // Generic error handling for CRUD operations
        if let Err(e) = result {
            println!("An error occurred: {}", e);
        }
    }
}
