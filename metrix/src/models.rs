use super::schema::metrics;
use serde_json;
use chrono::naive::NaiveDateTime;

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
