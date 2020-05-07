use chrono::naive::NaiveDateTime;

use diesel::sql_types::*;
use serde_json;
use rocket_contrib::json::Json;

use crate::schema::metrics;

#[derive(Queryable, QueryableByName)]
pub struct BucketResult {
    #[sql_type = "Double"]
    pub value: f64,
    #[sql_type = "Integer"]
    pub bucket_index: i32,
}

#[derive(Queryable, QueryableByName)]
pub struct MetricNameResult {
    #[sql_type = "Text"]
    pub metric_name: String
}

#[derive(Serialize, Deserialize)]
pub struct Bucket {
    pub value: f64,
    pub bucket: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Buckets {
    pub buckets: Vec<Bucket>,
}

#[derive(Serialize, Deserialize)]
pub struct BucketedData {
    pub data: Buckets,
}

#[derive(Serialize, Deserialize)]
pub struct MetricDataParamNames {
    pub parameter_names: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct MetricDataParams {
    pub data: MetricDataParamNames
}

#[derive(Serialize, Deserialize)]
pub struct MetricNames {
    pub metric_names: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct MetricNameParams {
    pub data: MetricNames
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="metrics"]
pub struct Metric {
    pub id: i32,
    pub metric_name: String,
    pub data: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="metrics"]
pub struct NewMetric {
    pub metric_name: String,
    pub data: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct ErrorObject {
    pub message: String,
    // pub code: usize,
}

#[derive(Serialize, Debug)]
pub struct Error {
    pub errors: Vec<ErrorObject>,
}

#[derive(Responder, Debug)]
pub enum ErrorResponder {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<Error>),

    #[response(status = 500, content_type = "json")]
    InternalServerError(Json<Error>),
}
