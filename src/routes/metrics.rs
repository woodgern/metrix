use chrono::naive::NaiveDateTime;
use std::mem::replace;

use diesel::prelude::*;
use diesel::sql_query;
use rocket::http::RawStr;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;

use crate::DbConn;
use crate::models::*;
use crate::parser::parse_parameter_name;
use crate::parser::parse_query_string;
use crate::schema::metrics;


#[post("/", data = "<metric_body>")]
pub fn create_metric_route(db_conn: DbConn, metric_body: Json<NewMetric>) -> Json<Metric> {
    let new_metric: NewMetric = metric_body.into_inner();

    let result: Metric = diesel::insert_into(metrics::table)
        .values(&new_metric)
        .get_result(&*db_conn)
        .expect("Error saving new metric");

    Json(result)
}

#[get("/?<offset>&<start_datetime>&<end_datetime>&<q>")]
pub fn query_metric_route(
    db_conn: DbConn,
    offset: Option<&RawStr>,
    start_datetime: Option<&RawStr>,
    end_datetime: Option<&RawStr>,
    q: Option<&RawStr>,
) -> Result<Json<Vec<Metric>>, BadRequest<Json<Error>>> {
    let filter_clause: String;
    let result = build_filter_clause(offset, start_datetime, end_datetime, q);
    match result {
        Ok(o) => {
            filter_clause = o;
        },
        Err(_) => {
            return Err(BadRequest(Some(Json(Error {
                errors: vec![ErrorObject { message: "bad value for `q` param".to_string() }]
            }))));
        },
    }

    println!("### QUERY: SELECT * FROM metrics {};", filter_clause);

    let query_string = format!("SELECT * FROM metrics {} ORDER BY id LIMIT 10", filter_clause);
    let results = sql_query(query_string)
        .load(&*db_conn)
        .expect("Error loading metrics");

    Ok(Json(results))
}

#[get("/search_metric_names?<q>")]
pub fn search_metric_names(db_conn: DbConn, q: &RawStr) -> Result<Json<MetricNameParams>, BadRequest<Json<Error>>> {
    let parsed_query : String;

    match q.url_decode() {
        Ok(query) => {
            parsed_query = query;
        },
        Err(_) => {
            return Err(BadRequest(Some(Json(Error {
                errors: vec![ErrorObject { message: "bad value for `q` param".to_string() }]
            }))));
        }
    }

    let query_string = format!("SELECT DISTINCT(metric_name) AS metric_name FROM metrics WHERE metric_name LIKE '%{}%' LIMIT 20", parsed_query);

    let query_result = sql_query(query_string)
        .load::<MetricNameResult>(&*db_conn)
        .expect("Error loading metrics");

    Ok(Json(MetricNameParams {
        data: MetricNames {
            metric_names: query_result.into_iter().map(|metric_name_obj| metric_name_obj.metric_name).collect()
        }
    }))
}

#[get("/search_parameters?<metric_name>")]
pub fn query_metric_params(db_conn: DbConn, metric_name: &RawStr) -> Result<Json<MetricDataParams>, BadRequest<Json<Error>>> {
    let result = metric_name.url_decode();
    let parsed_metric_name: String;

    match result {
        Ok(o) => {
            parsed_metric_name = o;
        },
        Err(_) => {
            return Err(BadRequest(Some(Json(Error {
                errors: vec![ErrorObject { message: "bad `metric_name` param".to_string() }]
            }))));
        }
    }

    let query_string = format!("SELECT * FROM metrics WHERE metric_name = '{}' ORDER BY id DESC LIMIT 1", parsed_metric_name);

    let query_result = sql_query(query_string)
        .load::<Metric>(&*db_conn)
        .expect("Error loading metrics");

    if query_result.len() == 0 {
        return Err(BadRequest(Some(Json(Error {
            errors: vec![ErrorObject { message: "no entry found for `metric_name`".to_string() }]
        }))));
    }

    let paths: Vec<String>;
    if query_result[0].data.is_object() {
        paths = get_paths_from_json(&query_result[0].data);
    } else {
        paths = vec![];
    }

    Ok(Json(MetricDataParams {
        data: MetricDataParamNames {
            parameter_names: paths
        }
    }))
}

fn get_paths_from_json(data: &serde_json::Value) -> Vec<String> {
    let mut output: Vec<Vec<String>> = vec![];
    let current_path = vec![];
    deep_keys(&data, current_path, &mut output);

    return output.into_iter().map(|keys| keys.join(".")).collect();
}

// https://stackoverflow.com/a/57275829
fn deep_keys(value: &serde_json::Value, current_path: Vec<String>, output: &mut Vec<Vec<String>>) {
    if current_path.len() > 0 {
        output.push(current_path.clone());
    }

    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let mut new_path = current_path.clone();
                new_path.push(k.to_owned());
                deep_keys(v, new_path, output);
            }
        },
        // Value::Array(array) => {
        //     for (i, v) in array.iter().enumerate() {
        //         let mut new_path = current_path.clone();
        //         new_path.push(i.to_string().to_owned());
        //         deep_keys(v,  new_path, output);
        //     }
        // },
        _ => ()
    }
}

#[get("/<aggregation>?<offset>&<start_datetime>&<end_datetime>&<q>&<bucket_count>&<metric_data_path>")]
pub fn aggregate_metrics_route(
    db_conn: DbConn,
    aggregation: Option<&RawStr>,
    offset: Option<&RawStr>,
    start_datetime: Option<&RawStr>,
    end_datetime: Option<&RawStr>,
    q: Option<&RawStr>,
    bucket_count: i32,
    metric_data_path: Option<&RawStr>,
) -> Result<Json<BucketedData>, BadRequest<Json<Error>>> {

    if !start_datetime.is_some() || !end_datetime.is_some() {
        return Err(BadRequest(Some(Json(Error {
            errors: vec![ErrorObject { message: "You need to send start_datetime and end_datetime".to_string() }]
        }))));
    }

    let filter_clause: String;
    let result = build_filter_clause(offset, start_datetime, end_datetime, q);
    match result {
        Ok(o) => {
            filter_clause = o;
        },
        Err(_) => {
            return Err(BadRequest(Some(Json(Error {
                errors: vec![ErrorObject { message: "bad value for `q` param".to_string() }]
            }))));
        },
    }

    let mut parameter_name = String::from("");
    if metric_data_path.is_some() {
        let param_name = metric_data_path.unwrap().url_decode();
        if param_name.is_ok() {
            let result = parse_parameter_name(param_name.ok().unwrap());
            match result {
                Ok(o) => {
                    parameter_name = format!("{}", o).to_string();
                },
                Err(_) => {
                    return Err(BadRequest(Some(Json(Error {
                        errors: vec![ErrorObject {
                            message: "malformatted `metric_data_path` param".to_string()
                        }]
                    }))));
                },
            }
        }
    }

    let start_timestamp = NaiveDateTime::parse_from_str(
        &start_datetime.unwrap().url_decode().ok().unwrap(),
        &"%Y-%m-%dT%H:%M:%S".to_string()
    ).ok().unwrap().timestamp();

    let end_timestamp = NaiveDateTime::parse_from_str(
        &end_datetime.unwrap().url_decode().ok().unwrap(),
        &"%Y-%m-%dT%H:%M:%S".to_string()
    ).ok().unwrap().timestamp();

    let bucket_size = (end_timestamp - start_timestamp) as f32 / bucket_count as f32;

    let aggregate: String;
    if aggregation.is_some() {
        let result = build_query_aggregate(aggregation.unwrap(), &parameter_name);
        match result {
            Ok(o) => {
                aggregate = o;
            },
            Err(_) => {
                return Err(BadRequest(Some(Json(Error {
                    errors: vec![ErrorObject {
                        message: "unknown aggregate type".to_string()
                    }]
                }))));
            }
        }
    } else {
        return Err(BadRequest(Some(Json(Error {
            errors: vec![ErrorObject {
                message: "No aggregate type provided".to_string()
            }]
        }))));
    }

    let query_string = format!(
        "SELECT
            ({}::DOUBLE PRECISION) as value,
            FLOOR((extract(epoch from created_at)-{})/{})::INTEGER as bucket_index
        FROM metrics {}
        GROUP BY bucket_index",
        aggregate,
        start_timestamp,
        bucket_size,
        filter_clause
    );

    println!("### QUERY: {}", query_string);

    let results = sql_query(query_string)
        .load::<BucketResult>(&*db_conn)
        .expect("Error loading metrics");

    let mut padded_results: Vec<Bucket> = vec![();bucket_count as usize]
        .iter().enumerate().map(
            |(i, _)|
            Bucket {
                value: 0.0,
                bucket: build_bucket_datetime(i as i64, bucket_size as i64, start_timestamp),
            }
        ).collect();

    for result in &results {
        replace(&mut padded_results[result.bucket_index as usize], Bucket {
            value: result.value,
            bucket: build_bucket_datetime(result.bucket_index as i64, bucket_size as i64, start_timestamp),
        });
    }
    return Ok(Json(
        BucketedData {
            data: Buckets {
                buckets: padded_results,
            }
        }
    ))
}

fn build_filter_clause(
    offset: Option<&RawStr>,
    start_datetime: Option<&RawStr>,
    end_datetime: Option<&RawStr>,
    q: Option<&RawStr>,
) -> Result<String, BadRequest<String>> {
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

    return Ok(filter_clause);
}

fn build_query_aggregate(aggregation: &str, parameter_name: &str) -> Result<String, String> {
    match aggregation {
        "count" => {
            Ok("COUNT(*)".to_string())
        },
        "max" => {
            Ok(format!("MAX(({})::NUMERIC)", parameter_name).to_string())
        },
        "min" => {
            Ok(format!("MIN(({})::NUMERIC)", parameter_name).to_string())
        },
        "avg" => {
            Ok(format!("AVG(({})::NUMERIC)", parameter_name).to_string())
        },
        "sum" => {
            Ok(format!("SUM(({})::NUMERIC)", parameter_name).to_string())
        },
        _ => {
            Err("Invalid aggregate specified".to_string())
        }
    }
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

fn build_bucket_datetime(bucket_index: i64, bucket_size: i64, start_timestamp: i64) -> NaiveDateTime {
    return NaiveDateTime::from_timestamp(bucket_index * bucket_size + start_timestamp, 0)
}
