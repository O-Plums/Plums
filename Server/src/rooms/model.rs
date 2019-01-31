use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::schema::rooms;
use crate::utils::database::Database;

#[derive(Queryable, Serialize, Identifiable ,Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: i32,
    pub topic: String,
    pub created_at: chrono::NaiveDateTime
}