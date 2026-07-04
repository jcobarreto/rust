mod models;
mod services;
mod screen;

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

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Fail when reading choice");

        match choice.trim() {
            "1" => display::create_client(),
            "2" => {
                if let Err(e) = display::list_clients() {
                    println!("Error listing clients: {}", e);
                }
            },
            "3" => {
                if let Err(e) = display::update_client() {
                    println!("Error updating client: {}", e);
                }
            },
            "4" => {
                if let Err(e) = display::delete_client() {
                    println!("Error deleting client: {}", e);
                }
            },
            "5" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid option, try again."),
        }
    }
}
