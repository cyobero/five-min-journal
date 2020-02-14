use super::schema::entries;
use rocket::config::Datetime;

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub title: String,
    pub question: String,
    pub answer: String,
    pub morning: bool,
    pub entry_date: Datetime,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry<'a> {
    pub title: &'a str,
    pub question: &'a str,
    pub answer: &'a str,
}

#[derive(Debug)]
pub struct EntryFormInput<'a> {
    title: &'a str,
    // can we make `question` a drop down?
    question: &'a str,
    answer: &'a str,
}
