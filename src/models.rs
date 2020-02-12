use std::time::SystemTime;

use rocket::http::RawStr;
use rocket::request::{FromForm, FromFormValue};

#[derive(Debug, FromFormValue)]
pub enum Question {
    grateful_for([String; 3]),
    make_today_great([String; 3]),
    affirrmation([String; 3]),
    amazing_thing([String; 3]),
    improve_today([String; 3]),
}

pub struct Entry {
    id: u8,
    title: Option<String>,
    question: Question,
    datetime: SystemTime,
}

#[derive(Debug, FromForm)]
pub struct EntryFormInput {
    id: u8,
    title: Option<String>,
    // can we make `question` a drop down?
    question: Question,
    #[from(field = "textarea")]
    answer: String,
    datetime: SystemTime,
}
