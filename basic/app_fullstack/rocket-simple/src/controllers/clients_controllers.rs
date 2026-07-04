use rocket_dyn_templates::{Template, context};
use crate::services::client_service;

use rocket::form::Form;
use rocket::response::{Redirect, Flash};
use crate::dtos::client_dto::ClientDTO;
use rocket::request::FlashMessage;

#[get("/clients")]
pub fn index() -> Template {
    let clients = client_service::get_clients();
    Template::render("clients/index", context! { clients: &clients })
}

#[get("/clients/new")]
pub fn new(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("clients/new", context!{ erro: erro_flash(flash) })
}

#[post("/clients/create", data = "<client_dto_form>")]
pub fn create(client_dto_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client_dto = client_dto_form.into_inner();

    if client_service::create_client(client_dto.name, client_dto.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to("/clients/new"),
            "Failed to create client",
        ))
    }
}

#[get("/clients/<id>/edit")]
pub fn edit(id: u32, flash: Option<FlashMessage<'_>>) -> Template {
    let client = client_service::get_client_by_id(id);
    Template::render("clients/edit", context! {
        client: &client,
        erro: erro_flash(flash)
    })
}

#[post("/clients/<id>/update", data = "<client_dto_form>")]
pub fn update(id: u32, client_dto_form: Form<ClientDTO>) -> Result<Redirect, Flash<Redirect>> {
    let client_dto = client_dto_form.into_inner();

    if client_service::update(id, client_dto.name, client_dto.cpf) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clients/{}/edit", id)),
            "Failed to update client",
        ))
    }
}

#[get("/clients/<id>/delete")]
pub fn delete(id: u32) -> Result<Redirect, Flash<Redirect>> {
    if client_service::delete_by_id(id) {
        Ok(Redirect::to("/clients"))
    } else {
        Err(Flash::error(
            Redirect::to(format!("/clients/{}/edit", id)),
            "Failed to delete client",
        ))
    }
}

fn erro_flash(flash: Option<FlashMessage<'_>>) -> String {
    let mut erro = "".to_string();
    if let Some(msg) = flash {
        if msg.kind() == "error" {
            erro = msg.message().to_string();
        }
    }
    erro
}
