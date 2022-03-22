use crate::ast::node::Node;

use super::{r#let::Let, expression::Expression};

pub enum Statement<'a> {
    Let(Let<'a>),
    Expression(Expression<'a>),
}