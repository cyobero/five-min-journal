use diesel::prelude::*;

use rocket_contrib::json::Json;

use std::collections::HashMap;

use crate::models::NewEntry;
use crate::DbConn;

use rocket::config::Datetime;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    question: &'static str,
    answer: &'static str,
    morning: bool,
    entry_date: Datetime,
}

#[get("/")]
pub fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[post("/entry", data = "<entry>")]
pub fn create(conn: DbConn, entry: Form<NewEntry>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::entries::table)
        .values(&entry.0)
        .execute(&conn.0)
        .map(|err| -> String {
            format!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("inserted {} row(s)", inserted_rows))
}
