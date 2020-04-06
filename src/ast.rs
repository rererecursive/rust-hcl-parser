use std::fmt::{Debug};

#[derive(Debug)]
pub struct Body {
    blocks: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub block_identifiers: Vec<String>,
}

