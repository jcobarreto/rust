use rocket::serde::json::Json;
use crate::model_views::admin_token::AdminToken;
use crate::model_views::erro_json::ErroJson;
use crate::dtos::login_dto::LoginDTO;
use crate::services::admin_service;
use rocket::response::status;
use rocket::http::Status;

#[post("/login", format = "json", data = "<login_dto_json>")]
pub fn login(login_dto_json: Json<LoginDTO>) -> Result<status::Custom<Json<AdminToken>>, status::Custom<Json<ErroJson>>> {
    let login_dto = login_dto_json.into_inner();

    match admin_service::login(login_dto.email, login_dto.password) {
        Ok(admin_token) => Ok(status::Custom(Status::Ok, Json(admin_token))),
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson { message: str_erro }))),
    }
}
