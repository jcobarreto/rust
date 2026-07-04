use crate::schema::clients;

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "clients"]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub phone: String,
}
