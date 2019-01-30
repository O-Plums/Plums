use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::schema::users;
use crate::utils::database::Database;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub phone_code: String,
    pub phone: String,
    pub username: Option<String>,
    pub room_id: Option<i32>,
    pub gender: Option<String>,
    pub validation_code: i32,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize, Debug)]
#[table_name="users"]
pub struct New {
    pub phone: String,
    pub phone_code: String,
    pub validation_code: i32
}

impl Message for New {
    type Result = Result<User, Error>;
}

impl Handler<New> for Database {
    type Result = Result<User, Error>;
    fn handle(&mut self, msg: New, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users).values(&msg).get_result::<User>(&self.0.get().unwrap())
    }
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[table_name="users"]
pub struct Find {
    pub phone: String,
    pub phone_code: String,
}

impl Message for Find {
    type Result = Result<Option<User>, Error>;
}

impl Handler<Find> for Database {
    type Result = Result<Option<User>, Error>;
    fn handle(&mut self, msg: Find, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let mut user = users.filter(phone.eq(&msg.phone)).filter(phone_code.eq(&msg.phone_code)).load::<User>(&self.0.get().unwrap()).expect("Error find user");
        Ok(user.pop())
    }
}