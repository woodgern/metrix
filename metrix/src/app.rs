// "Re-exports important traits and types.
// Meant to be glob imported when using Diesel."
use diesel::prelude::*;

use crate::lib::establish_connection;
use crate::schema::metrics;
use crate::models::*;
use crate::parser::parse_query_string;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use rocket::response::status::BadRequest;
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

#[get("/?<offset>&<start_datetime>&<end_datetime>&<q>")]
fn query_metric_route(
    offset: Option<&RawStr>,
    start_datetime: Option<&RawStr>,
    end_datetime: Option<&RawStr>,
    q: Option<&RawStr>,
) -> Result<Json<Vec<Metric>>, BadRequest<String>> {
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
        if is_valid_datetime_str(start_datetime.unwrap()) {
            filter_clause.insert_str(
                filter_clause.len(),
                &format!(
                    " AND created_at >= '{}'",
                    start_datetime.unwrap().url_decode().ok().unwrap()
                ).to_string()
            );
        }
    }

    if end_datetime.is_some() {
        if is_valid_datetime_str(end_datetime.unwrap()) {
            filter_clause.insert_str(
                filter_clause.len(),
                &format!(
                    " AND created_at <= '{}'",
                    end_datetime.unwrap().url_decode().ok().unwrap()
                ).to_string()
            );
        }
    }

    if q.is_some() {
        let query_string = q.unwrap().url_decode();
        if query_string.is_ok() {
            let result = parse_query_string(query_string.ok().unwrap());
            match result {
                Ok(o) => {
                    filter_clause.insert_str(
                        filter_clause.len(),
                        &format!(" AND {}", o).to_string()
                    );
                },
                Err(_) => {
                    return Err(BadRequest(Some("Malformatted query".to_string())))
                },
            }
        }
    }

    println!("### QUERY: SELECT * FROM metrics {};", filter_clause);

    let query_string = format!("SELECT * FROM metrics {} ORDER BY id LIMIT 10", filter_clause);
    let results = sql_query(query_string)
        .load(&db_conn)
        .expect("Error loading metrics");

    Ok(Json(results))
}

fn is_valid_datetime_str(raw_string: &RawStr) -> bool {
    let result = raw_string.url_decode();
    if result.is_err() {
        return false;
    }

    let datetime = result.ok().unwrap();
    let datetime_parsed = NaiveDateTime::parse_from_str(
        &datetime,
        &"%Y-%m-%dT%H:%M:%S".to_string() // "2014-5-17T12:34:56"
    );

    if datetime_parsed.is_ok() {
        return true;
    }

    false
}

pub fn create_app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/ping", routes![ping])
        .mount("/metrics", routes![create_metric_route, query_metric_route])
}
