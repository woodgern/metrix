#[derive(Queryable)]
pub struct Metric {
    pub id: i32,
    pub metric_name: String,
    pub body: String,
    pub created_at: String,
}
