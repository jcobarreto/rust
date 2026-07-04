extern crate rusqlite;

#[derive(Debug)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub phone: String,
}
