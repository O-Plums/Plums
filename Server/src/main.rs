extern crate uuid;
extern crate futures;
extern crate dotenv;
extern crate actix;
extern crate num_cpus;
extern crate actix_web;
extern crate chrono;
extern crate regex;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::env;
use dotenv::dotenv;
use actix::prelude::*;
use actix_web::{server, App, middleware};

pub mod schema;
pub mod users;
pub mod utils;

pub struct State {
    pub db: Addr<utils::database::Database>
}

fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,info,warn");

    let sys = System::new("plums");

    let pool = utils::database::pool();
    let addr = SyncArbiter::start(12, move || utils::database::Database(pool.clone()));
    let address = env::var("BIND_TO").expect("BIND_TO not set!");
    server::new(move || {
        let mut app = App::with_state(State { db: addr.clone() });
        app = app.middleware(middleware::Logger::default());        
        app = users::configure(app);
        app
    }).backlog(8192).workers(4).bind(&address).unwrap().start();

    let _ = sys.run();
}