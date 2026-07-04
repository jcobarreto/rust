use rocket::serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceDTO {
    pub title: String,
    pub description: String,
}
