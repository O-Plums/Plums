use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::users::model::{ User };
use crate::schema::messages;
use crate::utils::database::Database;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub user_id: i32;
    pub created_at: chrono::NaiveDateTime
}