use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}
