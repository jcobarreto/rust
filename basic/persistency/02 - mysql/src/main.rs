#[macro_use]
extern crate model_macro;

mod config;
mod models;
mod repositories;
mod screen;

use std::io;
use screen::display;

use crate::repositories::generic_repository;
use crate::models::client::Client;

fn main() {
    generic_repository::drop_table::<Client>().expect("Failed to drop client table");
    generic_repository::create_table::<Client>().expect("Failed to create client table");

    loop {
        println!("CRUD Clients");
        println!("1. Create Client");
        println!("2. List Clients");
        println!("3. Update Client");
        println!("4. Delete Client");
        println!("5. Exit");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Failed to read input");

        match escolha.trim() {
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
            _ => println!("Invalid option, please try again."),
        }
    }
}

// mod config;
// mod models;
// use std::result::Result;
// use models::client::Client;
// use mysql::prelude::Queryable;

// fn main() -> Result<(), mysql::Error> {
//     let mut cnn = config::cnn::get_connection()?;

//     let clients = cnn.query_map(
//         "SELECT id, name, phone FROM clients",
//         |(id, name, phone)| {
//             Client { id, name, phone }
//         },
//     )?;

//     for client in clients {
//         println!("------------------------------------------");
//         println!("ID: {}", client.id);
//         println!("Name: {}", client.name);
//         println!("Phone: {}", client.phone);
//     }
//     println!("------------------------------------------");

//     Ok(())
// }
