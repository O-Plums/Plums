extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate r2d2;
mod login;

use actix_web::http::{ Method };
use actix_web::{ middleware, pred, App, HttpResponse };
use actix::prelude::*;
use models::{ DbExecutor, AppState};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub fn create_new_router() -> App<AppState> {
    let manager = ConnectionManager::<PgConnection>::new("postgres://plums:password@localhost/plums");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to crate pool.");
    let addr = SyncArbiter::start(2, move || DbExecutor(pool.clone()));
    App::with_state(AppState{db: addr.clone()})
        .middleware(middleware::Logger::default())
        .resource("/login/{phone}", |r| r.method(Method::GET).with(login::Login))
        .default_resource(|r| {
            r.route().filter(pred::Not(pred::Get())).f(
                |_| HttpResponse::MethodNotAllowed());
        })
}