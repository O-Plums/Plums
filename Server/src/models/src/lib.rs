#![crate_name = "models"]

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::*;
use actix::{ Actor, SyncContext, Handler };
use actix_web::{ Error };
use actix::prelude::*;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_web;

pub mod schema;
pub mod rooms;
pub mod users;


pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

// Create new room
impl Handler<rooms::CreateRoom> for DbExecutor {
    type Result = Result<rooms::Room, Error>;

    fn handle(&mut self, msg: rooms::CreateRoom, _: &mut Self::Context) -> Self::Result {
        let new_room = rooms::NewRoom{ subject: &msg.subject };
        let room = diesel::insert_into(schema::rooms::table)
            .values(new_room)
            .get_result(&self.0.get().unwrap())
            .expect("Error saving new rooms");
        println!("handle new room {:?}", room);
        Ok(room)
    }
}

impl Handler<users::CreateUser> for DbExecutor {
    type Result = Result<users::User, Error>;

    fn handle(&mut self, input: users::CreateUser, _: &mut Self::Context) -> Self::Result {
        let new_user = users::NewUser{ phone: &input.phone, validation_code: &input.validation_code };
        let user = diesel::insert_into(schema::users::table)
            .values(new_user)
            .get_result(&self.0.get().unwrap())
            .expect("Error saving new users");
        println!("handle new user {:?}", user);
        Ok(user)
    }
}