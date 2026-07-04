use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminToken {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub token: String,
}
