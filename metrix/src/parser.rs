use std::str;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{take_while1, take_until, tag_no_case, tag},
    combinator::{map, opt},
    character::is_alphanumeric,
    character::complete::{digit1 as digit, multispace1 as multispace},
    sequence::{delimited, tuple},
    IResult,
};

pub struct Expression {
    left: Box<ExpressionType>,
    operator: String,
    right: Box<ExpressionType>,
}


pub enum ExpressionType {
    OuterExpression(Expression),
    BaseExpression(BaseExpression),
}

pub struct BaseExpression {
    field: FieldType,
    comparator: String,
    value: Value,
}

pub enum FieldType {
    RootField(Field),
    NestedField(Field),
    TerminalField(()),
}

pub struct Field {
    field_root: String,
    sub_fields: Box<FieldType>,
}

pub enum Value {
    String(String),
    Integer(String),
}

pub fn parse_query_string(input: String) -> Result<ExpressionType, &'static str> {
    println!("{:?}", input);
    match root_expression(&input.into_bytes()) {
        Ok((_, o)) => Ok(o),
        Err(_) => Err("Failed to parse query string"),
    }
}

pub fn parse_parameter_name(input: String) -> Result<FieldType, &'static str> {
    match parameter_name(&input.into_bytes()) {
        Ok((_, o)) => Ok(o),
        Err(_) => Err("Failed to parse parameter name"),
    }
}

impl fmt::Display for ExpressionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionType::OuterExpression(outer_expression) => write!(f, "{}", outer_expression),
            ExpressionType::BaseExpression(base_expression) => write!(f, "{}", base_expression),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{}", self.left)?;
        write!(f, " ")?;
        write!(f, "{}", self.operator)?;
        write!(f, " ")?;
        write!(f, "{}", self.right)?;
        write!(f, ")")
    }
}

impl fmt::Display for BaseExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Value::String(v) => {
                write!(f, "(")?;
                write!(f, "{}", self.field)?;
                write!(f, " ")?;
                write!(f, "{}", self.comparator)?;
                write!(f, " ")?;
                write!(f, "{}", v)?;
                write!(f, ")")
            },
            Value::Integer(i) => {
                write!(f, "(")?;
                write!(f, "CAST ({} AS INTEGER)", self.field)?;
                write!(f, " ")?;
                write!(f, "{}", self.comparator)?;
                write!(f, " ")?;
                write!(f, "{}", i)?;
                write!(f, ")")
            }
        }
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldType::RootField(field) => {
                write!(f, "{}", field.field_root)?;
                write!(f, "{}", field.sub_fields)
            },
            FieldType::NestedField(field) => {
                match *field.sub_fields {
                    FieldType::TerminalField(_) => {
                        write!(f, "->>")?;
                        write!(f, "'{}'", field.field_root)
                    },
                    FieldType::NestedField(_) => {
                        write!(f, "->")?;
                        write!(f, "'{}'", field.field_root)?;
                        write!(f, "{}", field.sub_fields)
                    },
                    _ => panic!("Root field not structure root")
                }
            },
            FieldType::TerminalField(_) => {
                write!(f, "")
            },
        }
    }
}

fn root_expression(s: &[u8]) -> IResult<&[u8], ExpressionType> {
    alt((
        map(
            tuple((
                base_expression,
                multispace,
                tag_no_case("or"),
                multispace,
                expression,
            )),
            |(left, _, _, _, right)| {
                ExpressionType::OuterExpression(
                    Expression {
                        operator: "or".to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                )
            }
        ),
        and_expression,
        base_expression,
    ))(s)
}

fn expression(s: &[u8]) -> IResult<&[u8], ExpressionType> {
    alt((
        map(
            tuple((
                base_expression,
                multispace,
                tag_no_case("or"),
                multispace,
                expression,
            )),
            |(left, _, _, _, right)| {
                ExpressionType::OuterExpression(
                    Expression {
                        operator: "or".to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                )
            },
        ),
        and_expression,
        base_expression,
    ))(s)
}

fn and_expression(s: &[u8]) -> IResult<&[u8], ExpressionType> {
    map(
        tuple((
            base_expression,
            multispace,
            tag_no_case("and"),
            multispace,
            expression,
        )),
        |(left, _, _, _, right)| {
            ExpressionType::OuterExpression(
                Expression {
                    operator: "and".to_string(),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            )
        },
    )(s)
}

fn base_expression(s: &[u8]) -> IResult<&[u8], ExpressionType> {
    map(
        tuple((
            parameter_name,
            opt_multispace,
            comparison_operator,
            opt_multispace,
            parameter_value,
        )),
        |(field, _, comparator, _, value)| {
            ExpressionType::BaseExpression(
                BaseExpression {
                    field,
                    comparator: str::from_utf8(comparator).unwrap().to_string(),
                    value,
                }
            )
        },
    )(s)
}

fn comparison_operator(s: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        tag_no_case("<="),
        tag_no_case(">="),
        tag_no_case("="),
        tag_no_case("<"),
        tag_no_case(">"),
    ))(s)
}

fn parameter_name(s: &[u8]) -> IResult<&[u8], FieldType> {
    alt((
        map(
            tuple((
                take_while1(is_sql_identifier),
                tag("."),
                sub_parameter_name,
            )),
            |(field, _, sub_fields)| {
                FieldType::RootField(
                    Field {
                        field_root: str::from_utf8(field).unwrap().to_string(),
                        sub_fields: Box::new(sub_fields),
                    }
                )
            }
        ),
        map(
            take_while1(is_sql_identifier),
            |field| {
                FieldType::RootField(
                    Field {
                        field_root: str::from_utf8(field).unwrap().to_string(),
                        sub_fields: Box::new(FieldType::TerminalField(())),
                    }
                )
            }
        ),
    ))(s)
}

fn sub_parameter_name(s: &[u8]) -> IResult<&[u8], FieldType> {
    alt((
        map(
            tuple((
                take_while1(is_sql_identifier),
                tag("."),
                sub_parameter_name,
            )),
            |(field, _, sub_fields)| {
                FieldType::NestedField(
                    Field {
                        field_root: str::from_utf8(field).unwrap().to_string(),
                        sub_fields: Box::new(sub_fields),
                    }
                )
            }
        ),
        map(
            take_while1(is_sql_identifier),
            |field| {
                FieldType::NestedField(
                    Field {
                        field_root: str::from_utf8(field).unwrap().to_string(),
                        sub_fields: Box::new(FieldType::TerminalField(())),
                    }
                )
            }
        ),
    ))(s)
}

fn parameter_value(s: &[u8]) -> IResult<&[u8], Value> {
    alt((
        map(
            digit,
            |d| {
                Value::Integer(
                    str::from_utf8(d).unwrap().to_string()
                )
            },
        ),
        map(
            delimited(opt(tag("'")), take_until("'"), opt(tag("'"))),
            |literal| {
                Value::String(
                    format!("'{}'", str::from_utf8(literal).unwrap().to_string())
                )
            },
        ),
    ))(s)
}

fn opt_multispace(s: &[u8]) -> IResult<&[u8], Option<&[u8]>> {
    opt(multispace)(s)
}

fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8
}
