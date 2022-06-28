use crate::ast::{node::Node, expression::{identifier::Identifier, Expression}};

pub enum Statement<'a> {
    Let {
        name: Identifier<'a>,
        expression: Box<Expression<'a>>,
    },
    Expression {
        expression: Box<Expression<'a>>,
    },
    Return {
        expression: Box<Expression<'a>>,
    },
}

impl<'a> Node for Statement<'a> {}