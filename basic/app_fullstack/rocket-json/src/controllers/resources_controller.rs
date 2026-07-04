use rocket::serde::json::Json;
use crate::models::resource::Resource;
use crate::model_views::erro_json::ErroJson;
use crate::dtos::resource_dto::ResourceDTO;
use crate::services::resource_service;
use rocket::response::status;
use rocket::http::Status;

#[get("/resources")]
pub fn index() -> Json<Vec<Resource>> {
    let resources = resource_service::resource_list();
    Json(resources)
}

#[post("/resources", format = "json", data = "<resource_dto_json>")]
pub fn create(resource_dto_json: Json<ResourceDTO>) -> Result<status::Custom<Json<Resource>>, status::Custom<Json<ErroJson>>> {
    let resource_dto = resource_dto_json.into_inner();

    match resource_service::create_resource(resource_dto) {
        Ok(resource) => Ok(status::Custom(Status::Created, Json(resource))),
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson { message: str_erro }))),
    }
}

#[put("/resources/<id>", format = "json", data = "<resource_dto_json>")]
pub fn update(id: i32, resource_dto_json: Json<ResourceDTO>) -> Result<status::Custom<Json<Resource>>, status::Custom<Json<ErroJson>>> {
    let resource_dto = resource_dto_json.into_inner();

    match resource_service::update_resource(id, resource_dto) {
        Ok(resource) => Ok(status::Custom(Status::Ok, Json(resource))),
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson { message: str_erro }))),
    }
}

#[get("/resources/<id>", format = "json")]
pub fn show(id: i32) -> status::Custom<Json<Resource>> {
    let resource = resource_service::get_by_id(id);
    status::Custom(Status::Ok, Json(resource))
}

#[delete("/resources/<id>")]
pub fn delete(id: i32) -> Result<status::Custom<Json<()>>, status::Custom<Json<ErroJson>>> {
    match resource_service::delete_resource(id) {
        Ok(_) => Ok(status::Custom(Status::NoContent, Json(()))),
        Err(str_erro) => Err(status::Custom(Status::BadRequest, Json(ErroJson { message: str_erro }))),
    }
}
