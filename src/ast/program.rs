use super::{statement::Statement, node::Node};

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program {
            statements: Vec::new(),
        }
    }
}

impl<'a> Node for Program<'a> {}