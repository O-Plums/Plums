#![allow(unused_variables)]
extern crate uuid;
extern crate futures;
extern crate dotenv;
extern crate actix;
extern crate num_cpus;
extern crate actix_web;
extern crate chrono;
extern crate regex;
extern crate byteorder;
extern crate bytes;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;
extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::env;
use dotenv::dotenv;
use actix::*;
use actix_web::{server, App, middleware};

pub mod schema;
pub mod users;
pub mod rooms;
pub mod chat;
pub mod themes;
pub mod topics;
pub mod messages;
pub mod utils;

pub struct DBstate(Addr<utils::database::Database>);
pub struct Wsstate(Addr<utils::websocket::ChatServer>);

pub struct State {
    pub db: DBstate,
    pub ws: Wsstate
}

fn main() {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,info,warn");

    let sys = System::new("plums");

    let pool = utils::database::pool();
    let addrDb = SyncArbiter::start(12, move || utils::database::Database(pool.clone()));
    let addWs = Arbiter::start(| _ | utils::websocket::ChatServer::default());
    let address = env::var("BIND_TO").expect("BIND_TO not set!");
    server::new(move || {
        let mut app = App::with_state(State { db: DBstate(addrDb.clone()), ws: Wsstate(addWs.clone()) });
        app = app.middleware(middleware::Logger::default());        
        app = chat.configure(app);
        app = users::configure(app);
        app
    }).backlog(8192).workers(4).bind(&address).unwrap().start();

    let _ = sys.run();
}