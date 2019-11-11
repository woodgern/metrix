#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
// #[macro_use] extern crate diesel_migrations;
extern crate rocket_contrib;
extern crate serde_json;

// use serde_json::json;

// needed for Diesel stuff?
// "Re-exports important traits and types. Meant to be glob imported when using Diesel."
use diesel::prelude::*;

// embed_migrations!();

pub mod lib;
pub mod models;
pub mod schema;

use lib::establish_connection;
use schema::metrics;
use models::*;
use rocket_contrib::json::Json;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[get("/")]
fn root_ping() -> &'static str {
    "pong"
}


#[post("/metrics", data = "<metric_body>")]
fn create_metric_route(metric_body: Json<NewMetric>) -> String {
    let new_metric = NewMetric { ..metric_body.into_inner() };
    let db_conn = establish_connection();

    let result: Metric = diesel::insert_into(metrics::table)
        .values(&new_metric)
        .get_result(&db_conn)
        .expect("Error saving new metric");

    "dont\n".to_string()
}

fn main() {
    println!("### Enter the Metrix ###");
    // let db_conn = establish_connection();

    println!("### running db migrations...");
    // let result = embedded_migrations::run(&db_conn);
    // println!("### migration done; result: {}", result.is_ok());

    rocket::ignite()
        .mount("/ping", routes![root_ping])
        .mount("/metrix", routes![ping, create_metric_route])
        .launch();
}
