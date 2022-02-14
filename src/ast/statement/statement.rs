use crate::ast::node::Node;

use super::r#let::Let;

pub enum Statement<'a> {
    Let(Let<'a>),
}

impl<'a> Node for Statement<'a> {}