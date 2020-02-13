use std::containers::HashMap;

use crate::models::Entry;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}
