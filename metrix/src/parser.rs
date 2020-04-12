use std::str;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{take_while1, tag},
    combinator::map,
    character::is_alphanumeric,
    character::complete::{digit1 as digit, multispace1 as multispace},
    sequence::tuple,
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

// This uses the proper nom 5 approach, as opposed to the outdated nom 4 approach like the rest
// of the parser. The rest will need to be refactored.
pub fn parse_parameter_name(input: String) -> Result<FieldType, &'static str> {
    match param_name(&input.into_bytes()) {
        Ok((_, o)) => Ok(o),
        Err(_) => Err("Failed to parse parameter name"),
    }
}

fn param_name(s: &[u8]) -> IResult<&[u8], FieldType> {
    alt((
        map(
            tuple((
                take_while1(is_sql_identifier),
                tag("."),
                sub_param_name,
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

fn sub_param_name(s: &[u8]) -> IResult<&[u8], FieldType> {
    alt((
        map(
            tuple((
                take_while1(is_sql_identifier),
                tag("."),
                sub_param_name,
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

named!(root_expression<ExpressionType>,
    complete!(expression)
);

named!(expression<ExpressionType>,
    alt!(
        do_parse!(
            left: base_expression >>
            multispace >>
            tag_no_case!("or") >>
            multispace >>
            right: expression >>
            (ExpressionType::OuterExpression(
                Expression {
                    operator: "or".to_string(),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            ))
        )
        | and_expression
        | base_expression
    )
);

named!(and_expression<ExpressionType>,
    do_parse!(
        left: base_expression >>
        multispace >>
        tag_no_case!("and") >>
        multispace >>
        right: expression >>
        (ExpressionType::OuterExpression(
            Expression {
                operator: "and".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            }
        ))
    )
);

named!(base_expression<ExpressionType>,
    do_parse!(
        field: parameter_name >>
        opt_multispace >>
        comparator: comparison_operator >>
        opt_multispace >>
        value: parameter_value >>
        (ExpressionType::BaseExpression(
            BaseExpression {
                field,
                comparator: str::from_utf8(comparator).unwrap().to_string(),
                value,
            }
        ))
    )
);

named!(comparison_operator,
    alt!(
          do_parse!(op: tag_no_case!("<=") >> (op))
        | do_parse!(op: tag_no_case!(">=") >> (op))
        | do_parse!(op: tag!("=") >> (op))
        | do_parse!(op: tag!("<") >> (op))
        | do_parse!(op: tag!(">") >> (op))
    )
);

named!(parameter_name<FieldType>,
    alt!(
          do_parse!(
            field: take_while1!(is_sql_identifier) >>
            tag!(".") >>
            sub_fields: sub_parameter_name >>
            (FieldType::RootField(
                Field {
                    field_root: str::from_utf8(field).unwrap().to_string(),
                    sub_fields: Box::new(sub_fields),
                }
            ))
          )
        | do_parse!(
            field: take_while1!(is_sql_identifier) >>
            (FieldType::RootField(
                Field {
                    field_root: str::from_utf8(field).unwrap().to_string(),
                    sub_fields: Box::new(FieldType::TerminalField(())),
                }
            ))
        )
    )
);

named!(sub_parameter_name<FieldType>,
    alt!(
          do_parse!(
            field: take_while1!(is_sql_identifier) >>
            tag!(".") >>
            sub_fields: sub_parameter_name >>
            (FieldType::NestedField(
                Field {
                    field_root: str::from_utf8(field).unwrap().to_string(),
                    sub_fields: Box::new(sub_fields),
                }
            ))
          )
        | do_parse!(
            field: take_while1!(is_sql_identifier) >>
            (FieldType::NestedField(
                Field {
                    field_root: str::from_utf8(field).unwrap().to_string(),
                    sub_fields: Box::new(FieldType::TerminalField(())),
                }
            ))
        )
    )
);

named!(parameter_value<Value>,
    alt!(
          do_parse!(
              d: digit >>
              (Value::Integer(
                str::from_utf8(d).unwrap().to_string()
              ))
          )
        | do_parse!(
            literal: delimited!(opt!(tag!("'")), take_until!("'"), opt!(tag!("'"))) >>
            (Value::String(
                format!("'{}'", str::from_utf8(literal).unwrap().to_string())
            ))
        )
    )
);

named!(opt_multispace<Option<&[u8]>>,
    opt!(multispace)
);

fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8
}
