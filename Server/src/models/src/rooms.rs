use super::schema::rooms;
use actix::{ Message };
use actix_web::{ Error };

#[derive(Queryable, Identifiable, PartialEq, Debug, Serialize)]
#[table_name="rooms"]
pub struct Room {
    pub id: i32,
    pub subject: String
}

#[derive(Insertable)]
#[table_name="rooms"]
pub struct NewRoom<'a> {
    pub subject: &'a str
}

pub struct CreateRoom {
    pub subject: String
}

impl Message for CreateRoom {
    type Result = Result<Room, Error>;
}
