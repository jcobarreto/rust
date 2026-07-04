use crate::schema::clients;

#[derive(Insertable)]
#[table_name="clients"]
pub struct NewClient {
    pub name: String,
    pub phone: String,
}
