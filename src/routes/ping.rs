#[get("/")]
pub fn ping() -> &'static str {
    "pong"
}
