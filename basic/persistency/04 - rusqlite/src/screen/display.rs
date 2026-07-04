use std::error::Error;
use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::repositories::client_repository;
use crate::config::cnn::establish_connection;

pub fn create_client() -> Result<(), Box<dyn Error>> {
    clear_screen();
    let mut name = String::new();
    let mut phone = String::new();

    println!("Client Name: ");
    io::stdin().read_line(&mut name)?;
    println!("Client Phone: ");
    io::stdin().read_line(&mut phone)?;

    let conn = establish_connection();
    client_repository::create(&conn, &name.trim(), &phone.trim())?;

    println!("Client successfully registered.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn list_clients() -> Result<(), Box<dyn Error>> {
    clear_screen();
    let conn = establish_connection();
    let clients = client_repository::list(&conn)?;
    for client in clients {
        println!("----------------------------------");
        println!("ID: {}", client.id);
        println!("Name: {}", client.name);
        println!("Phone: {}", client.phone);
    }
    println!("----------------------------------");

    pause_for_enter();
    clear_screen();

    Ok(())
}

// The update_client and delete_client functions are similar to the create_client function, but they handle updating and deleting clients respectively. They also use the establish_connection function to get a database connection and call the appropriate repository functions to perform the operations.

pub fn update_client() -> Result<(), Box<dyn Error>> {
    clear_screen();
    let mut id = String::new();
    let mut name = String::new();
    let mut phone = String::new();

    println!("ID of the Client to be updated: ");
    io::stdin().read_line(&mut id)?;
    let id = id.trim().parse::<i32>()?;

    println!("New name of the Client: ");
    io::stdin().read_line(&mut name)?;
    println!("New phone of the Client: ");
    io::stdin().read_line(&mut phone)?;

    let conn = establish_connection();
    client_repository::update(&conn, id, &name.trim(), &phone.trim())?;

    println!("Client successfully updated.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn delete_client() -> Result<(), Box<dyn Error>> {
    clear_screen();
    let mut id = String::new();

    println!("ID of the Client to be deleted: ");
    io::stdin().read_line(&mut id)?;
    let id = id.trim().parse::<i32>()?;

    let conn = establish_connection();
    client_repository::delete(&conn, id)?;

    println!("Client successfully deleted.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

fn pause_for_enter() {
    println!("Press Enter to continue...");
    let mut _descartar = String::new(); // Temporary variable, prefixed with _ to indicate it is intentionally unused.
    io::stdin().read_line(&mut _descartar).expect("Failed to read input");
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
