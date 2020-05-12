#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod routes;

fn main() {
    rocket::ignite().mount("/", 
    routes![ 
        routes::frontend::index,
        routes::frontend::files,
        routes::api::files,
    ]).launch();
  }