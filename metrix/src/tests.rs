use super::create_app;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn ping_me_baby() {
  let client = Client::new(create_app()).expect("valid rocket instance");

  let mut response = client.get("/ping").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string(), Some("pong".into()));
}
