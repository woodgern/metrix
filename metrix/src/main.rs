#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_migrations;
extern crate chrono;
extern crate rocket_contrib;
extern crate serde_json;

// needed for Diesel stuff?
// "Re-exports important traits and types. Meant to be glob imported when using Diesel."
use diesel::prelude::*;

embed_migrations!();

pub mod lib;
pub mod models;
pub mod schema;

use lib::establish_connection;
use schema::metrics;
use models::*;
use rocket_contrib::json::Json;
use rocket::http::RawStr;


#[get("/")]
fn ping() -> &'static str {
    "pong"
}

#[post("/", data = "<metric_body>")]
fn create_metric_route(metric_body: Json<NewMetric>) -> Json<Metric> {
    let new_metric: NewMetric = metric_body.into_inner();
    let db_conn = establish_connection();

    let result: Metric = diesel::insert_into(metrics::table)
        .values(&new_metric)
        .get_result(&db_conn)
        .expect("Error saving new metric");

    Json(result)
}

#[get("/?<offset>")]
fn query_metric_route(offset: Option<&RawStr>) -> Json<Vec<Metric>> {
    let db_conn = establish_connection();
    let mut metric_id: i32 = 0;

    if offset.is_some() {
        let result = offset.unwrap().url_decode();
        // https://api.rocket.rs/v0.3/rocket/http/struct.RawStr.html
        if result.is_ok() {
            metric_id = result.ok().unwrap().parse().unwrap();
        }
    }

    let results = metrics::table.filter(metrics::id.gt(metric_id))
        .order(metrics::id)
        .limit(10)
        .load::<Metric>(&db_conn)
        .expect("Error loading metrics");

    Json(results)
}

fn main() {
    println!("### Enter the Metrix ###");
    let db_conn = establish_connection();

    println!("### running db migrations...");
    let result = embedded_migrations::run(&db_conn);
    println!("### migration done; result: {}", result.is_ok());

    rocket::ignite()
        .mount("/ping", routes![ping])
        .mount("/metrics", routes![create_metric_route, query_metric_route])
        .launch();
}
