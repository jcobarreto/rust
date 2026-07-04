use crate::services::client_service;
use std::io;
use std::process::Command;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

pub fn create_client() {
  clear_screen();
  let mut name = String::new();
  let mut phone = String::new();

  println!("Client Name: ");
  io::stdin().read_line(&mut name).expect("Fail reading name");

  println!("Client Phone: ");
  io::stdin().read_line(&mut phone).expect("Fail reading phone");

  client_service::create(&name.trim(), &phone.trim()).expect("Fail creating client.");
  clear_screen();
}

pub fn list_clients() -> Result<(), std::io::Error> {
  clear_screen();
  let clients = client_service::list()?;
  for client in &clients {
    println!("------------------------------------------");
    println!("ID: {}", client.id);
    println!("Name: {}", client.name);
    println!("Phone: {}", client.phone);
  }
  println!("------------------------------");
  pause_screen();
  clear_screen();

  Ok(())
}

fn pause_screen() {
  println!("Press Enter to continue...");
  let mut _discard = String::new();
  io::stdin().read_line(&mut _discard).expect("Failed to pause screen");
  clear_screen();
}

fn clear_screen() {
  if cfg!(target_os = "windows") {
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
  } else {
    Command::new("clear").status().unwrap();
  }
}

fn sleep_seconds(seconds: u64) {
  thread::sleep(Duration::from_secs(seconds));
}

pub fn update_client() -> Result<(), std::io::Error> {
  clear_screen();
  let mut id_input = String::new();
  let mut name = String::new();
  let mut phone = String::new();

  println!("Client ID to update: ");
  io::stdin().read_line(&mut id_input).expect("Fail reading ID");
  let id = Uuid::parse_str(id_input.trim()).expect("Invalid ID");

  println!("New Client Name: ");
  io::stdin().read_line(&mut name).expect("Fail reading name");

  println!("New Client Phone: ");
  io::stdin().read_line(&mut phone).expect("Fail reading phone");

  client_service::update(id, &name.trim(), &phone.trim()).expect("Fail updating client.");

  println!("Client updated successfully!");
  sleep_seconds(2);
  clear_screen();

  Ok(())
}

pub fn delete_client() -> Result<(), std::io::Error> {
  clear_screen();
  let mut id_input = String::new();

  println!("Client ID to delete: ");
  io::stdin().read_line(&mut id_input).expect("Fail reading ID");
  let id = Uuid::parse_str(id_input.trim()).expect("Invalid ID");

  client_service::delete(id).expect("Fail deleting client.");

  println!("Client deleted successfully!");
  sleep_seconds(2);
  clear_screen();

  Ok(())
}
