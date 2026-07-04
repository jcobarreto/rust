use rusqlite::{params, Connection, Result};

use crate::models::client::Client;

// Cria um novo cliente
pub fn create(conn: &Connection, name_str: &str, phone_str: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO clients (name, phone) VALUES (?1, ?2)",
        params![name_str, phone_str],
    )?;
    Ok(())
}

// Lista todos os clientes
pub fn list(conn: &Connection) -> Result<Vec<Client>> {
    let mut stmt = conn.prepare("SELECT id, name, phone FROM clients")?;
    let clients_iter = stmt.query_map(params![], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            phone: row.get(2)?,
        })
    })?;

    clients_iter.collect()
}

// Atualiza um cliente existente
pub fn update(conn: &Connection, client_id: i32, new_name: &str, new_phone: &str) -> Result<()> {
    conn.execute(
        "UPDATE clients SET name = ?1, phone = ?2 WHERE id = ?3",
        params![new_name, new_phone, client_id],
    )?;
    Ok(())
}

// Exclui um cliente
pub fn delete(conn: &Connection, client_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM clients WHERE id = ?1",
        params![client_id],
    )?;
    Ok(())
}
