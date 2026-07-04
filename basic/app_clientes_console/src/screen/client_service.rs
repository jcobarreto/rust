use crate::models::client::Client;
use super::{ read::*, utils::{ clear_screen, wait_for } };

pub fn add_client(clients: &mut Vec<Client>) {
  clear_screen();

  let mut client: Client = Client::default();

  client.id = clients.len() as usize + 1;

  // println!("Enter client name:");
  // client.name = read_data();

  // println!("Enter client CPF:");
  // client.cpf = read_data();

  // println!("Enter client address:");
  // client.address = read_data();

  input_client_data(&mut client);

  clients.push(client);

  clear_screen();
  println!("Client added successfully!");
  wait_for(1);
}

fn input_client_data(client: &mut Client) {
  println!("Enter client name:");
  client.name = read_data();

  println!("Enter client CPF:");
  client.cpf = read_data();

  println!("Enter client address:");
  client.address = read_data();
}

pub fn edit_client(clients: &mut Vec<Client>) {
  clear_screen();
  if check_clients_exists(clients) {
    return;
  }
  let id: usize = capture_client_id();
  if let Some((index, client)) = get_client_by_id(clients, &id) {
    println!("{}", "-".repeat(40));
    println!("Editing client...");
    println!("{}", "-".repeat(40));
    show_client(client);
    println!("{}", "-".repeat(40));
    input_client_data(&mut clients[index]);
    clear_screen();
    println!("Client updated successfully!");
  } else {
    clear_screen();
    println!("Client not found.");
  }
}

pub fn remove_client(clients: &mut Vec<Client>) {
  clear_screen();
  if check_clients_exists(clients) {
    return;
  }
  let id: usize = capture_client_id();
  if let Some((index, client)) = get_client_by_id(clients, &id) {
    println!("{}", "-".repeat(40));
    show_client(client);
    println!("{}", "-".repeat(40));
    println!("Are you sure you want to remove this client? (y/n)");
    let confirmation = read_data();
    if confirmation.to_lowercase() != "y" {
      clear_screen();
      println!("Client removal cancelled.");
      wait_for(1);
      return;
    }
    clients.remove(index);
    clear_screen();
    println!("Client removed successfully!");
    wait_for(1);
  } else {
    clear_screen();
    println!("Client not found.");
    wait_for(1);
  }
}

fn get_client_by_id<'a>(clients: &'a Vec<Client>, id: &usize) -> Option<(usize, &'a Client)> {
  clients.iter().enumerate().find(|(_, client)| client.id == *id)
}

fn capture_client_id() -> usize {
  clear_screen();
  println!("Enter client ID:");
  read_data_int()
}

pub fn list_clients(clients: &Vec<Client>) {
  clear_screen();

  if check_clients_exists(clients) {
    return;
  }

  println!("{}", "-".repeat(40));
  println!("Registered Clients:");
  println!("{}", "-".repeat(40));
  for client in clients {
    show_client(client);
    println!("{}", "-".repeat(40));
  }
  println!("Type any key to return to the menu...");
  read_data();

}

pub fn show_client(client: &Client) {
  println!("\
    ID: {}\n\
    Name: {}\n\
    CPF: {}\n\
    Address: {}
  ", client.id, client.name, client.cpf, client.address)
}

fn check_clients_exists(clients: &[Client]) -> bool {
  if clients.len() == 0 {
    println!("No clients registered.");
    wait_for(1);
    return true;
  }

  return false;
}
