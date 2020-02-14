use rocket::config::Datetime;
use rocket::request::FromForm;

use crate::schema::entries;

use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub question: String,
    pub answer: String,
    pub morning: bool,
    pub entry_date: Datetime,
}

#[derive(Insertable, FromForm, Deserialize)]
#[table_name = "entries"]
pub struct NewEntry {
    pub title: String,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Insertable, FromForm, Serialize)]
pub struct NewEntryForm {
    title: String,
    question: String,
    answer: String,
}
