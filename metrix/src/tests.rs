use super::create_app;
use rocket::local::Client;
use rocket::http::Status;

use crate::parser::parse_query_string;

#[test]
fn ping_me_baby() {
  let client = Client::new(create_app()).expect("valid rocket instance");

  let mut response = client.get("/ping").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string(), Some("pong".into()));
}

#[test]
fn parse_query_string_single_expression_integer_value() {
  let query_string = "data.user.id = 625";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(CAST (data->'user'->>'id' AS INTEGER) = 625)");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_string_value() {
  let query_string = "data.user.name = 'Billiam'";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(data->'user'->>'name' = 'Billiam')");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_non_json_field() {
  let query_string = "metric_name = 'CREATE USER'";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(metric_name = 'CREATE USER')");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_multiple_expression_with_and() {
  let query_string = "metric_name = 'CREATE USER' and body.id = 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "((metric_name = 'CREATE USER') and (CAST (body->>'id' AS INTEGER) = 12))");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_multiple_expression_with_or() {
  let query_string = "metric_name = 'CREATE USER' or metric_name = 'UPDATE USER' and body.id = 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(
        format!("{}", o),
        "((metric_name = 'CREATE USER') or ((metric_name = 'UPDATE USER') and (CAST (body->>'id' AS INTEGER) = 12)))"
      );
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_less_than_operator() {
  let query_string = "data.user.id < 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(CAST (data->'user'->>'id' AS INTEGER) < 12)");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_greater_than_operator() {
  let query_string = "data.user.id > 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(CAST (data->'user'->>'id' AS INTEGER) > 12)");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_less_than_equals_operator() {
  let query_string = "data.user.id <= 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(CAST (data->'user'->>'id' AS INTEGER) <= 12)");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}

#[test]
fn parse_query_string_single_expression_greater_than_equals_operator() {
  let query_string = "data.user.id >= 12";

  let parsed_query = parse_query_string(query_string.to_string());

  match parsed_query {
    Ok(o) => {
      assert_eq!(format!("{}", o), "(CAST (data->'user'->>'id' AS INTEGER) >= 12)");
    },
    Err(_) => {
      panic!("Unable to parse query string")
    },
  }
}
