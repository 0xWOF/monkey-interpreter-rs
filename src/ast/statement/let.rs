use crate::ast::expression::{identifier::Identifier, Expression};

pub struct Let<'a> {
    pub name: Identifier<'a>,
    pub value: Expression<'a>,
}