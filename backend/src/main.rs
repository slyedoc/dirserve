#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// 1.
use crate::routes::{ static_files, get };
mod routes;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            // 2.
            routes![
                static_files::file,
                get::index,
            ],
        )
}

fn main() {
    rocket().launch();
}