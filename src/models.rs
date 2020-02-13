use super::schema::entries;

use std::time::SystemTime;

use rocket::http::RawStr;
use rocket::request::{FromForm, FromFormValue};

#[derive(Debug, FromFormValue)]
pub enum Question {
    grateful_for(String),
    make_today_great(String),
    affirrmation(String),
    amazing_thing(String),
    improve_today(String),
}

#[derive(FromForm, Queryable)]
pub struct Entry {
    pub id: u8,
    pub title: Option<String>,
    pub question: Question,
    pub answer: String,
    pub entry_date: SystemTime,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry<'a> {
    pub title: &'a Option<String>,
    pub question: &'a Question,
    pub answer: &'a str,
}

#[derive(Debug, FromForm)]
pub struct EntryFormInput {
    title: Option<String>,
    // can we make `question` a drop down?
    question: Question,
    answer: String,
}
