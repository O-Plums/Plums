use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::rooms::model::{ Room };
use crate::schema::users;
use crate::utils::database::Database;

#[derive(Queryable, Serialize, Associations, Deserialize, Identifiable ,Debug)]
#[table_name="users"]
#[belongs_to(Room)]
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

impl User {
    pub fn to_update(&self) -> Update {
        Update {
            id: self.id.clone(),
            phone_code: self.phone_code.clone(),
            phone: self.phone.clone(),
            username: self.username.clone(),
            room_id: self.room_id.clone(),
            gender: self.gender.clone(),
            validation_code: self.validation_code.clone(),
        }
    }
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
        let mut user = users.filter(phone.eq(&msg.phone))
            .filter(phone_code.eq(&msg.phone_code))
            .load::<User>(&self.0.get().unwrap())
            .expect("Error find user");
        Ok(user.pop())
    }
}

#[derive(Insertable, Debug, Clone)]
#[table_name="users"]
pub struct Update {
    id: i32,
    pub phone_code: String,
    pub phone: String,
    pub username: Option<String>,
    pub room_id: Option<i32>,
    pub gender: Option<String>,
    pub validation_code: i32,
}


impl Message for Update {
    type Result = Result<User, Error>;
}

impl Handler<Update> for Database {
    type Result = Result<User, Error>;
    fn handle(&mut self, input: Update, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        diesel::update(users.find(input.id))
        .set((
            phone.eq(input.phone),
            phone_code.eq(input.phone_code),
            username.eq(input.username),
            room_id.eq(input.room_id),
            gender.eq(input.gender),
            validation_code.eq(input.validation_code)
        )).get_result::<User>(&self.0.get().unwrap())
    }
}
