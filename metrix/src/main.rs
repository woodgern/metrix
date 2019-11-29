#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel_migrations;
extern crate chrono;
extern crate rocket_contrib;
extern crate serde_json;

embed_migrations!();

pub mod lib;
pub mod models;
pub mod schema;
pub mod app;

use lib::establish_connection;

pub fn create_app() -> rocket::Rocket {
  app::create_app()
}

fn main() {
    println!("### Enter the Metrix ###");
    let db_conn = establish_connection();

    println!("### running db migrations...");
    let result = embedded_migrations::run(&db_conn);
    println!("### migration done; result: {}", result.is_ok());

    create_app().launch();
}

#[cfg(test)]
mod test {
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
}
