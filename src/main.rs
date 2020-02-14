#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod routes;
pub mod schema;

use rocket_contrib::templates::Template;

// Registers your database with Rocket, returning a `fairing` that can be `.attach`'d to your
// Rocket application to set up a connection pool for it and automatically manage it for you.
#[database("fivemj")]
pub struct DbConn(diesel::MysqlConnection);

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![routes::index])
        .attach(Template::fairing())
        .attach(DbConn::fairing())
}

fn main() {
    rocket().launch();
}
