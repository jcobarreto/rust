#[macro_use] extern crate rocket;

mod controllers;
mod model_views;
mod models;
mod services;
mod dtos;
mod middlewares;

use controllers::{ home_controller, resources_controller, login_controller };

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(middlewares::auth_guard::JwtFairing)
    .mount("/", routes![
        home_controller::index,
        home_controller::unauthorized,
        login_controller::login,

        resources_controller::index,
        resources_controller::create,
        resources_controller::update,
        resources_controller::show,
        resources_controller::delete,
    ])
}
