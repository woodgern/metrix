use super::schema::metrics;
use serde_json;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Queryable)]
// #[table_name="metrics"] -- idk why it throws compile error
// "error: cannot find attribute `table_name` in this scope" ...
pub struct Metric {
    pub id: i32,
    pub metric_name: String,
    pub data: serde_json::Value,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="metrics"]
pub struct NewMetric {
    pub metric_name: String,
    pub data: serde_json::Value,
}
