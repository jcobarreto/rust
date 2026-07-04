use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;

use diesel::result::Error as DieselError;
use crate::repositories::client_repository;
use crate::config::cnn::establish_connection;

pub fn create_client() -> Result<(), DieselError> {
    clear_screen();
    let mut nome = String::new();
    let mut phone = String::new();

    println!("Client Name: ");
    io::stdin().read_line(&mut nome).expect("Failed to read name");

    println!("Client Phone: ");
    io::stdin().read_line(&mut phone).expect("Failed to read phone");

    let conn = establish_connection();
    client_repository::create(&conn, &nome.trim(), &phone.trim())?;

    println!("Client successfully registered");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn list_clients() -> Result<(), DieselError> {
    clear_screen();
    let conn = establish_connection();
    let clients = client_repository::list(&conn)?;
    for client in clients {
        println!("------------------------------------------");
        println!("ID: {}", client.id);
        println!("Name: {}", client.name);
        println!("Phone: {}", client.phone);
    }
    println!("------------------------------------------");

    pause_until_enter();
    clear_screen();

    Ok(())
}

pub fn update_client() -> Result<(), DieselError> {
    clear_screen();
    let mut id = String::new();
    let mut name = String::new();
    let mut phone = String::new();

    println!("ID of the Client to be updated: ");
    io::stdin().read_line(&mut id).expect("Failed to read ID");
    let id = id.trim().parse::<i32>().expect("Invalid ID");

    println!("New name of the Client: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("New phone of the Client: ");
    io::stdin().read_line(&mut phone).expect("Failed to read phone");

    let conn = establish_connection();
    client_repository::update(&conn, id, &name.trim(), &phone.trim())?;

    println!("Client successfully updated.");
    pause_for_seconds(2);
    clear_screen();

    Ok(())
}

pub fn delete_client() -> Result<(), DieselError> {
    clear_screen();
    let mut id = String::new();

    println!("ID of the Client to be deleted: ");
    io::stdin().read_line(&mut id).expect("Failed to read ID");
    let id = id.trim().parse::<i32>().expect("Invalid ID");

    let conn = establish_connection();
    client_repository::delete(&conn, id)?;

    println!("Client successfully deleted.");
    pause_for_seconds(2);
    clear_screen();

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
