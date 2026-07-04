use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use crate::schema::clients;

use crate::models::client::Client;
use crate::models::new_client::NewClient;

pub fn create(conn: &MysqlConnection, name_str: &str, phone_str: &str) -> Result<(), Error> {
    let new_client = NewClient {
        name: name_str.to_string(),
        phone: phone_str.to_string(),
    };

    diesel::insert_into(clients::table)
        .values(&new_client)
        .execute(conn)?;

    Ok(())
}

pub fn list(conn: &MysqlConnection) -> Result<Vec<Client>, Error> {
    clients::table.load::<Client>(conn)
}

pub fn update(conn: &MysqlConnection, id_client: i32, new_name: &str, new_phone: &str) -> Result<(), Error> {
    diesel::update(clients::table.find(id_client))
        .set((clients::name.eq(new_name), clients::phone.eq(new_phone)))
        .execute(conn)?;

    Ok(())
}

pub fn delete(conn: &MysqlConnection, id_client: i32) -> Result<(), Error> {
    diesel::delete(clients::table.find(id_client))
        .execute(conn)?;

    Ok(())
}
