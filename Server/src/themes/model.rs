use chrono;
use diesel;
use diesel::prelude::*;
use actix::prelude::*;
use diesel::result::Error;

use crate::schema::themes;
use crate::utils::database::Database;

#[derive(Queryable, Identifiable ,Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime
}