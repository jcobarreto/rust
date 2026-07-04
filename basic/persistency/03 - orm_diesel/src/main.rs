mod models;
mod screen;
mod config;
mod repositories;

use std::io;
use screen::display;

#[macro_use]
extern crate diesel;
mod schema;

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
            "1" => screen::display::create_client(),
            "2" => screen::display::list_clients(),
            "3" => screen::display::update_client(),
            "4" => screen::display::delete_client(),
            "5" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid option, please try again.");
                Ok(()) // Retorna Ok para manter a consistência de tipo
            },
        };

        // Generic error handling for CRUD operations
        if let Err(e) = result {
            println!("An error occurred: {}", e);
        }
    }

    // let mut name = String::new();
    // let mut phone = String::new();

    // println!("Enter client name:");
    // io::stdin().read_line(&mut name).expect("Failed to read line");
    // println!("Enter client phone:");
    // io::stdin().read_line(&mut phone).expect("Failed to read line");

    // let conn = config::cnn::establish_connection();
    // client_repository::create(&conn, &name.trim(), &phone.trim()).expect("Error creating client");

    // let clients = client_repository::list(&conn).expect("Error listing clients");
    // println!("Clients in the database:");
    // for client in clients {
    //     println!("------------------------------------------");
    //     println!("ID: {}", client.id);
    //     println!("Name: {}", client.name);
    //     println!("Phone: {}", client.phone);
    // }
    // println!("------------------------------------------");

}
