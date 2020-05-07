#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

pub mod db;
pub mod models;
pub mod parser;
pub mod schema;

mod routes;

// This registers your database with Rocket, returning a `Fairing` that can be `.attach`'d to your
// Rocket application to set up a connection pool for it and automatically manage it for you.
#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/ping", routes![
            routes::ping::ping,
        ])
        .mount("/metrics", routes![
            routes::metrics::create_metric_route,
            routes::metrics::query_metric_route,
            routes::metrics::aggregate_metrics_route,
            routes::metrics::query_metric_params,
            routes::metrics::search_metric_names
        ])
        .attach(DbConn::fairing())
}
