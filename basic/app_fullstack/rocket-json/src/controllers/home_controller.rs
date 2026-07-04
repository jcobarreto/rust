use rocket::serde::json::Json;
use crate::model_views::home::Home;
use crate::model_views::erro_json::ErroJson;
use rocket::response::status;
use rocket::http::Status;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(Home {
        message: "Welcome to the home page!".to_string(),
        endpoints: vec![
            "/resources".to_string(),
        ]
    })
}

#[get("/unauthorized")]
pub fn unauthorized() -> status::Custom<Json<ErroJson>> {
    status::Custom(Status::Unauthorized, Json(ErroJson {
        message: "Not authorized to access this area".to_string(),
    }))
}
