use super::schema::metrics;
use serde_json;
// use diesel::types::Timestamp;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name="metrics"]
pub struct Metric {
    // pub id: i32,
    pub metric_name: String,
    pub data: serde_json::Value,
    // pub created_at: SystemTime,
}
