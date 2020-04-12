mod common;

use rocket::http::Status;

use common::create_app_client;

#[test]
fn ping_me_baby() {
  let client = create_app_client();

  let mut response = client.get("/ping").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string(), Some("pong".into()));
}
