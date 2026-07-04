use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
}
