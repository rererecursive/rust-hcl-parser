use std::fmt::{Debug};

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Block {
        block_identifiers: Vec<String>,
        attributes: Vec<Statement>,
        child_blocks: Vec<Statement>,
    },

    Attribute {
        identifier: String,
    }
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub node: T,
}

pub type Statement = Located<StatementType>;
