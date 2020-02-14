#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use crate::models::{Entry, NewEntry};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}

pub fn insert_default_values(conn: &MysqlConnection) -> QueryResult<usize> {
    use schema::entries::dsl::*;

    diesel::insert_into(entries).default_values().execute(conn)
}
