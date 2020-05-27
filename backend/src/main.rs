#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use structopt::StructOpt;
use rocket_contrib::serve::StaticFiles;

mod routes;

#[derive(StructOpt, Debug)]
#[structopt(name = "dirserve")]
struct Opt {

    #[structopt(short, long, default_value = "9000")]
    port: u16,

   
}

fn main() {
    let opt = Opt::from_args();

    let cfg = rocket::config::Config::build(rocket::config::Environment::Production)
        .port(opt.port)       
        .unwrap();


    rocket::custom(cfg)
    .mount("/",
        routes![
            routes::root::redirect,

        ],
    )
    .mount("/public",
        routes![ 
            routes::frontend::index,
            routes::frontend::files,
        ],
    )
    .mount("/api",
        routes![

            routes::api::files,
        ],
    )
    .mount("/files/",
        StaticFiles::from(".")
    )
    .launch();
  }