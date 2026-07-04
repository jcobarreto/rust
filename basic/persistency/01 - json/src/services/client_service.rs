use crate::models::client::Client;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek};
use uuid::Uuid;

const CLIENTS_JSON: &str = "db/clients.json";

pub fn create(name: &str, phone: &str) -> std::io::Result<()> {
  let mut file_reader = File::open(CLIENTS_JSON).unwrap_or_else(|_| File::create(CLIENTS_JSON).unwrap());
  let mut data = String::new();
  file_reader.read_to_string(&mut data)?;
  drop(file_reader);   // Close the file after reading

  let mut clients: Vec<Client> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());

  let new_client = Client {
    id: Uuid::new_v4(),
    name: name.to_string(),
    phone: phone.to_string(),
  };
  clients.push(new_client);

  // Serialize the updated clients list and write it back to the file
  let data_json = serde_json::to_string_pretty(&clients)?;

  let mut file_writer = OpenOptions::new().write(true).truncate(true).open(CLIENTS_JSON)?;
  file_writer.write_all(data_json.as_bytes())?;

  Ok(())
}

pub fn list() -> std::io::Result<Vec<Client>> {
  let mut file = File::open(CLIENTS_JSON)?;

  let mut data = String::new();
  file.read_to_string(&mut data)?;

  let clients: Vec<Client> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
  Ok(clients)
}

pub fn update(client_id: Uuid, name: &str, phone: &str) -> std::io::Result<()> {
  let mut clients = list()?;

  if let Some(client) = clients.iter_mut().find(|c| c.id == client_id) {
    client.name = name.to_string();
    client.phone = phone.to_string();
  } else {
    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Client not found"));
  }

  // Serialize the updated clients list and write it back to the file
  let data_json = serde_json::to_string_pretty(&clients)?;
  let mut file = OpenOptions::new().read(true).write(true).open(CLIENTS_JSON)?;
  file.set_len(0)?; // Clear the file before writing
  file.seek(std::io::SeekFrom::Start(0))?; // Move the cursor to the beginning of the file
  file.write_all(data_json.as_bytes())?;

  Ok(())
}

pub fn delete(client_id: Uuid) -> std::io::Result<()> {
  let mut clients = list()?;

  clients.retain(|c| c.id != client_id);

  // Serialize the updated clients list and write it back to the file
  let data_json = serde_json::to_string_pretty(&clients)?;
  let mut file = OpenOptions::new().read(true).write(true).open(CLIENTS_JSON)?;
  file.set_len(0)?; // Clear the file before writing
  file.seek(std::io::SeekFrom::Start(0))?; // Move the cursor to the beginning of the file
  file.write_all(data_json.as_bytes())?;

  Ok(())
}
