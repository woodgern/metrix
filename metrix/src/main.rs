#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;
// #[macro_use] extern crate diesel_migrations;

// embed_migrations!();

pub mod lib;

// use lib::establish_connection;
// use metrix::schema::metrics::dsl::*;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[post("/metrics")]
fn create_metrics() -> &'static str {
    "metrics"
}

fn main() {
    println!("### Enter the Metrix ###");
    // let db_conn = establish_connection();

    println!("### running db migrations...");
    // let result = embedded_migrations::run(&db_conn);
    // println!("### migration done; result: {}", result.is_ok());

    rocket::ignite().mount("/metrix", routes![ping, create_metrics]).launch();
}
