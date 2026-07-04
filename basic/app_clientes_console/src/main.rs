mod screen;
mod models;

use models::client::Client;
use screen::menu as menu;

fn main() {
  let mut clients: Vec<Client> = Vec::new();
  menu::show_menu(&mut clients);
}