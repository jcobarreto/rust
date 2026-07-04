use crate::models::client::Client;
use crate::repositories::generic_repository;
use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn create_client() {
    clear_screen();
    let mut name = String::new();
    let mut phone = String::new();

    println!("Name of the Client: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Phone of the Client: ");
    io::stdin().read_line(&mut phone).expect("Failed to read phone");

    let client = Client {
        id: 0, // The ID will be auto-incremented by the database, so we can set it to 0 or any placeholder value.
        name: name.trim().to_string(),
        phone: phone.trim().to_string(),
    };

    generic_repository::insert(&client).expect("Failed to create client");

    println!("Client created successfully.");
    pause_for_seconds(2);
    clear_screen();
}

pub fn list_clients() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    let clients: Vec<Client> = generic_repository::list()?;
    for client in clients {
        println!("------------------------------------------");
        println!("ID: {}", client.id);
        println!("Name: {}", client.name);
        println!("Phone: {}", client.phone);
    }
    println!("------------------------------------------");

    pause_until_enter();

    Ok(())
}

fn pause_until_enter() {
    println!("Press Enter to continue...");
    let mut _discard = String::new(); // Temporary variable, prefixed with _ to indicate it is intentionally unused.
    io::stdin().read_line(&mut _discard).expect("Failed to read input");
    clear_screen();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}

fn pause_for_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

pub fn update_client() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    let mut id = String::new();
    let mut name = String::new();
    let mut phone = String::new();

    println!("ID of the Client to be updated: ");
    io::stdin().read_line(&mut id).expect("Failed to read ID");
    let id = id.trim().parse::<u32>().expect("Invalid ID");

    println!("New name of the Client: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("New phone of the Client: ");
    io::stdin().read_line(&mut phone).expect("Failed to read phone");

    let client = Client {
        id: id,
        name: name.trim().to_string(),
        phone: phone.trim().to_string(),
    };

    generic_repository::update(client.id, &client).expect("Failed to update client");

    println!("Client updated successfully.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn delete_client() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    let mut id = String::new();

    println!("ID of the Client to be deleted: ");
    io::stdin().read_line(&mut id).expect("Failed to read ID");
    let id = id.trim().parse::<u32>().expect("Invalid ID");

    generic_repository::delete::<Client>(id).expect("Failed to delete client");

    println!("Client deleted successfully.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}
