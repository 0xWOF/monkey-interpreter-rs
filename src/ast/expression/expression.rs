use crate::ast::node::Node;

use super::{identifier::Identifier, integer::Integer, prefix::Prefix};

pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Integer(Integer),
    Prefix(Prefix<'a>),
}

impl<'a> Node for Expression<'a> {}