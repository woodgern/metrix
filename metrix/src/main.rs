#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
// #[macro_use] extern crate diesel_migrations;
extern crate rocket_contrib;
extern crate serde_json;

use serde_json::json;

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
fn create_metric_route(metric_body: Json<Metric>) -> String {
    // "metrics".to_string()
    let new_metric = Metric { ..metric_body.into_inner() };
    // let db_conn = establish_connection();

    use diesel::prelude::*;
    let db_conn = PgConnection::establish(
        &"postgres://user:stompy@db:5432/metrix".to_string())
        .expect(&format!("Error connecting to {}", "database_url"));

    diesel::insert_into(metrics::table)
        .values(&new_metric)
        .execute(&db_conn)
        .expect("Error saving new metric");

    "dont".to_string()
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
