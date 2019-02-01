use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;

use crate::users::model::{ User };
use crate::schema::{ messages };

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, Debug, Message)]
#[belongs_to(User)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime
}