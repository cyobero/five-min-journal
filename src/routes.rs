use std::collections::HashMap;

use rocket::config::Datetime;
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
