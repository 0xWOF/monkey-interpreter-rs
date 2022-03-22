use crate::ast::expression;

pub struct Expression<'a> {
    pub expression: Box<expression::Expression<'a>>,
}