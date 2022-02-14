use crate::ast::node::Node;

use super::identifier::Identifier;

pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}

impl<'a> Node for Expression<'a> {}