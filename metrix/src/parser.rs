#[macro_use]
extern crate nom;

use std::str;
use std::fmt;

use nom::{
    character::is_alphanumeric,
    character::complete::{digit1 as digit, multispace1 as multispace},
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
    field: String,
    comparator: String,
    value: String,
}

pub fn parse_query_string(input: String) -> Result<ExpressionType, &'static str> {
    match root_expression(input.into_bytes()) {
        Ok((_, o)) => Ok(o),
        Err(_) => Err("Failed to parse query string"),
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
        write!(f, "(")?;
        write!(f, "{}", self.field)?;
        write!(f, " ")?;
        write!(f, "{}", self.comparator)?;
        write!(f, " ")?;
        write!(f, "{}", self.value)?;
        write!(f, ")")
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
                field: str::from_utf8(field).unwrap().to_string(),
                comparator: str::from_utf8(comparator).unwrap().to_string(),
                value: value,
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

named!(parameter_name,
    do_parse!(
        param: take_while1!(is_sql_identifier) >>
        (param)
    )
);

named!(parameter_value<String>,
    alt!(
          do_parse!(
              s: digit >>
              tag!(".") >>
              e: digit >>
              (
                format!("{}.{}",
                    str::from_utf8(s).unwrap().to_string(),
                    str::from_utf8(e).unwrap().to_string(),
                )
              )
          )
        | do_parse!(
              d: digit >>
              (str::from_utf8(d).unwrap().to_string())
          )
        | do_parse!(
            literal: delimited!(opt!(tag!("\"")), take_until!("\""), opt!(tag!("\""))) >>
            (format!("\"{}\"", str::from_utf8(literal).unwrap().to_string()))
        )
    )
);

named!(opt_multispace<Option<&[u8]>>,
    opt!(multispace)
);

fn is_sql_identifier(chr: u8) -> bool {
    is_alphanumeric(chr) || chr == '_' as u8 || chr == '.' as u8
}
