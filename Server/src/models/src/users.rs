use super::schema::users;
use actix::{ Message };
use actix_web::{ Error };



#[derive(Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub phone: String,
    pub validation_code: i32,
    pub room_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub phone: &'a str,
    pub validation_code: &'a i32,
}

pub struct CreateUser {
    pub phone: String,
    pub validation_code: i32,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}