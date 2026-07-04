use crate::models::client::Client;
use crate::screen::read::*;
use crate::screen::utils::{ clear_screen };
use crate::screen::client_service::{add_client, edit_client, remove_client, list_clients};

pub fn show_menu(clients: &mut Vec<Client>) {
  loop {
    clear_screen();

    println!("\
    ============ Client Menu ============\n\
    Choose an option:\n\n\
    1 - Register new client\n\
    2 - Edit client\n\
    3 - Remove client\n\
    4 - List clients\n\
    0 - Exit\n
    ");

    let option = read_data_int();
    match option {
      1 => add_client(clients),
      2 => edit_client(clients),
      3 => remove_client(clients),
      4 => list_clients(clients),
      0 => {
        println!("Exiting...");
        break;
      },
      _ => println!("Invalid option, please try again."),
    }
  }
}
