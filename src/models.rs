use super::schema::entries;
use rocket::config::Datetime;
use rocket::request::FromForm;

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub question: String,
    pub answer: String,
    pub morning: bool,
    pub entry_date: Datetime,
}

#[derive(Insertable, FromForm)]
#[table_name = "entries"]
pub struct NewEntry {
    pub title: String,
    pub question: String,
    pub answer: String,
}
