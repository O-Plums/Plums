use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::themes::model::{ Theme }
use crate::schema::topics;
use crate::utils::database::Database;

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug)]
#[belongs_to(Theme)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub id: i32,
    pub name: String,
    pub theme: i32;
    pub created_at: chrono::NaiveDateTime
}