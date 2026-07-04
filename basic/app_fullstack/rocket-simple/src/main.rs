#[macro_use] extern crate rocket;
mod models;
mod services;
mod dtos;

mod controllers {
    pub mod home_controllers;
    pub mod clients_controllers;
}

use rocket_dyn_templates::Template;
use controllers::home_controllers;
use controllers::clients_controllers;
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            home_controllers::index,

            clients_controllers::index,
            clients_controllers::new,
            clients_controllers::create,
            clients_controllers::edit,
            clients_controllers::update,
            clients_controllers::delete,
        ])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
