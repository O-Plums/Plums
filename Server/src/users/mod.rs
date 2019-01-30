use std::env;
use actix_web::{ http::Method, App };

use super::State;
pub mod model;
pub mod controllers;

pub fn configure(app: App<State>) -> App<State> {
    app.scope("/users", |scope| {
        scope.resource("/signup", |r| r.method(Method::GET).with(controllers::Signup::get))
    }).resource("/login", |r| r.method(Method::POST).with(controllers::Login::post))
}