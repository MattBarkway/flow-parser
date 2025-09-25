use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SchemaNode {
    pub prefix: String,
    pub model: HashMap<String, FieldType>,
    pub children: Vec<SchemaNode>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
}

#[derive(PartialEq, Debug)]
pub struct DecodedFlow {
    pub prefix: String,
    pub contents: Vec<String>,
    pub children: Vec<DecodedFlow>,
}

#[derive(Debug)]
pub struct Node {
    pub(crate) prefix: String,
    pub(crate) contents: Vec<String>,
    pub(crate) children: Vec<usize>, // store child indices
}