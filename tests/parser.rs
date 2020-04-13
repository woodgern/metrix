use metrix::parser::parse_query_string;
use metrix::parser::parse_parameter_name;


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

#[test]
fn parse_parameter_name_raises_error() {
  let metric_name = "".to_string();
  assert_eq!(parse_parameter_name(metric_name).unwrap_err(), "Failed to parse parameter name");
}

#[test]
fn parse_parameter_name_no_nesting() {
  let metric_name = "data";

  let parsed_name = parse_parameter_name(metric_name.to_string());

  match parsed_name {
    Ok(o) => {
      assert_eq!(format!("{}", o), "data");
    },
    Err(_) => {
      panic!("Unable to parse parameter name")
    },
  }
}

#[test]
fn parse_parameter_name_single_nested_parameter() {
  let metric_name = "data.height";

  let parsed_name = parse_parameter_name(metric_name.to_string());

  match parsed_name {
    Ok(o) => {
      assert_eq!(format!("{}", o), "data->>'height'");
    },
    Err(_) => {
      panic!("Unable to parse parameter name")
    },
  }
}

#[test]
fn parse_parameter_name_heavily_nested_parameter() {
  let metric_name = "one.two.three.four.five.six.seven.eight.nine.ten";

  let parsed_name = parse_parameter_name(metric_name.to_string());
  match parsed_name {
    Ok(o) => {
      assert_eq!(format!("{}", o), "one->'two'->'three'->'four'->'five'->'six'->'seven'->'eight'->'nine'->>'ten'");
    },
    Err(_) => {
      panic!("Unable to parse parameter name")
    },
  }
}
