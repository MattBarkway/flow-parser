use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SchemaNode {
    pub prefix: String,
    pub model: HashMap<String, FieldType>,
    pub children: Vec<SchemaNode>,
}

impl SchemaNode {
    pub fn new(prefix: &str, model: HashMap<String, FieldType>, children: Vec<SchemaNode>) -> SchemaNode {
        SchemaNode {
            prefix: "ROOT".to_string(),
            model: HashMap::new(),
            children,
        }
    }
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
    pub prefix: String,
    pub contents: Vec<String>,
    pub children: Vec<usize>,
}

impl Node {
    pub fn new(prefix: &str, contents: Vec<&str>) -> Self {
        Node {
            prefix: prefix.to_string(),
            contents: contents.into_iter().map(String::from).collect(),
            children: Vec::new(),
        }
    }
}

pub trait NodeArena {
    fn push_node(&mut self, prefix: &str, contents: Vec<&str>) -> usize;
}

impl NodeArena for Vec<Node> {
    fn push_node(&mut self, prefix: &str, contents: Vec<&str>) -> usize {
        let idx = self.len();
        self.push(Node::new(prefix, contents));
        idx
    }
}