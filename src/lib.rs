#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde;

pub mod models;
pub mod schema;

use self::models::{Entry, EntryFormInput, NewEntry, Question};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_entry<'a>(
    conn: &MysqlConnection,
    title: Option<&'a str>,
    question: &'a Question,
    answer: &'a str,
) -> Entry {
    use schema::entries;

    let new_entry = NewEntry {
        title: title,
        question: question,
        answer: answer,
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new entry.")
}
