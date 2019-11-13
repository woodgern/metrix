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
use chrono::naive::NaiveDateTime;
use diesel::sql_query;


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

#[get("/?<offset>&<start_datetime>&<end_datetime>")]
fn query_metric_route(
    offset: Option<&RawStr>,
    start_datetime: Option<&RawStr>,
    end_datetime: Option<&RawStr>,
) -> Json<Vec<Metric>> {
    let db_conn = establish_connection();
    let mut filter_clause = String::from("WHERE 1=1");

    if offset.is_some() {
        let result = offset.unwrap().url_decode();
        // https://api.rocket.rs/v0.3/rocket/http/struct.RawStr.html
        if result.is_ok() {
            let metric_id_str: String = result.ok().unwrap();
            filter_clause.insert_str(
                filter_clause.len(),
                &format!(" AND id > {}", metric_id_str).to_string()
            );
        }
    }

    if start_datetime.is_some() {
        let result = start_datetime.unwrap().url_decode();
        if result.is_ok() {
            let created_at_start = result.ok().unwrap();
            let created_at_start_parsed = NaiveDateTime::parse_from_str(
                &created_at_start,
                &"%Y-%m-%dT%H:%M:%S".to_string() // "2014-5-17T12:34:56"
            );

            if created_at_start_parsed.is_ok() {
                filter_clause.insert_str(
                    filter_clause.len(),
                    &format!(" AND created_at > '{}'", created_at_start).to_string()
                );
            }
        }
    }

    if end_datetime.is_some() {
        let result = end_datetime.unwrap().url_decode();
        if result.is_ok() {
            let created_at_end = result.ok().unwrap();
            let created_at_end_parsed = NaiveDateTime::parse_from_str(
                &created_at_end,
                &"%Y-%m-%dT%H:%M:%S".to_string() // "2014-5-17T12:34:56"
            );

            if created_at_end_parsed.is_ok() {
                filter_clause.insert_str(
                    filter_clause.len(),
                    &format!(" AND created_at < '{}'", created_at_end).to_string()
                );
            }
        }
    }

    println!("### QUERY: SELECT * FROM metrics {};", filter_clause);

    let query_string = format!("SELECT * FROM metrics {} ORDER BY id LIMIT 10", filter_clause);
    let results = sql_query(query_string)
        .load(&db_conn)
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
