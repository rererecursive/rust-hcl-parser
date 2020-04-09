use std::fmt::{Debug};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Hcl {
    pub statements: Vec<Statement>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StatementType {
    Block {
        block_identifiers: Vec<String>,
        attributes: Vec<Statement>,
        child_blocks: Vec<Statement>,
    },

    Attribute {
        identifier: String,
        expression: Expression
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ExpressionType {
    Null, // Temp.

    Text {
        text: String,
    },

    TemplateInterpolation {
        text: String,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TermType {
    Boolean {
        value: bool,
    },

    Null,

    Number {
        value: u32,
    },

    Tuple {
        expressions: Vec<Expression>,
    },

    FunctionCall {
        name: String,
        expressions: Vec<Expression>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Located<T> {
    pub node: T,
}

pub type Statement = Located<StatementType>;
pub type Expression = Located<ExpressionType>;
pub type Term = Located<TermType>;
