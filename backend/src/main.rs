#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use structopt::StructOpt;

mod routes;

#[derive(StructOpt, Debug)]
#[structopt(name = "dirserve")]
struct Opt {

    #[structopt(short, long, default_value = "9000")]
    port: u16,

   
}

fn main() {
    let opt = Opt::from_args();

    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .port(opt.port)       
        .unwrap();


    rocket::custom(cfg)
    .mount("/", 
        routes![ 
            routes::frontend::index,
            routes::frontend::files,
            routes::api::files,
        ]
    ).launch();
  }