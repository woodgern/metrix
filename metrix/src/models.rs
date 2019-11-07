#[derive(Serialize, Deserialize, Queryable)]
pub struct Metric {
    pub id: Option<i32>,
    pub metric_name: String,
    pub body: String,
    pub created_at: Option<String>,
}
