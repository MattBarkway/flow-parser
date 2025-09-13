use crate::errors::FlowParseError;
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SchemaNode {
    pub prefix: String,
    pub model: HashMap<String, FieldType>,
    pub children: Vec<SchemaNode>,
}

impl SchemaNode {
    fn to_map(&self, map: &mut HashMap<String, SchemaNode>) {
        map.insert(self.prefix.clone(), self.clone());

        // Recurse into children
        for child in &self.children {
            child.to_map(map);
        }
    }
}

#[derive(Debug, Clone)]
struct SchemaMapNode {
    model: HashMap<String, FieldType>,
    children: HashMap<String, SchemaMapNode>,
}

impl From<SchemaNode> for SchemaMapNode {
    fn from(node: SchemaNode) -> Self {
        Self {
            model: node.model,
            children: node
                .children
                .into_iter()
                .map(|child| {
                    let prefix = child.prefix.clone();
                    (prefix, SchemaMapNode::from(child))
                })
                .collect(),
        }
    }
}
fn schema_to_map(schema: Vec<SchemaNode>) -> HashMap<String, Vec<String>> {
    schema
        .into_iter()
        .map(|node| {
            let prefix = node.prefix.clone();
            (prefix, node.children.into_iter().map(|child| {child.prefix}).collect())
        })
        .collect()
}


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
}

pub struct DecodedFlow {
    pub prefix: String,
    pub contents: Vec<String>,
    pub children: Vec<DecodedFlow>,
}

type SchemaMap = HashMap<String, SchemaNode>;

pub fn load_schema(path: &str) -> Result<Vec<SchemaNode>, FlowParseError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).map_err(|e| FlowParseError::Serde(e))?)
}

fn gen_top_level_map(nodes: &[SchemaNode]) -> HashMap<String, SchemaNode> {
    nodes
        .iter()
        .map(|node| (node.prefix.clone(), node.clone()))
        .collect()
}

fn separate_prefix<'a>(line: &'a str) -> Result<(&'a str, Vec<&'a str>), FlowParseError> {
    let line = line.trim_end_matches('|');
    let mut split = line.splitn(2, '|');
    let prefix = split.next().ok_or(FlowParseError::Invalid("Missing prefix".to_owned()))?;
    let contents: Vec<&'a str> = split
        .next()
        .map(|rest| rest.split('|').collect())
        .ok_or(FlowParseError::Invalid("Missing contents".to_owned()))?;
    Ok((prefix, contents))

}

#[derive(Debug)]
struct Node {
    prefix: String,
    contents: Vec<String>,
    children: Vec<usize>, // store child indices
}

pub fn parse<'a, I>(schema: Vec<SchemaNode>, mut content: I) -> Result<DecodedFlow, FlowParseError>
where
    I: Iterator<Item = &'a str>,
{
    let mut nodes = Vec::new();
    nodes.push(Node {
        prefix: "ROOT".to_string(),
        contents: vec![],
        children: vec![],
    });

    let mut stack = vec![0];

    let schema_map = schema_to_map(schema);
    while let Some(row) = content.next() {
        let (prefix, contents) = separate_prefix(row)?;

        let node = Node {
            prefix: prefix.to_owned(),
            contents: contents.into_iter().map(String::from).collect(),
            children: Vec::new(),
        };

        let idx = nodes.len();
        nodes.push(node);

        // while let Some(&parent_idx) = stack.last() {
        //     let candidate_node = &nodes[parent_idx];
        //     if candidate_node.prefix == "ROOT" {
        //         // ROOT node always counts as valid parent
        //         // Only present in stack on first row
        //     }
        //     // Find current location in schema
        //     // By looking up prefixes
        //     if schema_map
        //         .get(&nodes[parent_idx].prefix)
        //         .and_then(|n| n.children.get(prefix))
        //         .is_some()
        //     {
        //         nodes[parent_idx].children.push(idx);
        //         break;
        //     } else {
        //         stack.pop();
        //     }
        // }
        //
        // if schema.contains_key(prefix.as_str()) {
        //     stack.push(idx);
        // }


        // if let Some(sub_schema) = schema_map.get(prefix) {
        //     if !sub_schema.children.is_empty() {
        //         stack.push(node)
        //     }
        // } else { continue }
    }

    Ok(DecodedFlow {
        prefix: "Foo".to_owned(),
        contents: Vec::new(),
        children: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_basic() {
    //     // Setup test data
    //     let schema = vec![]; // or some SchemaNode objects
    //     let content = vec!["foo".to_string(), "bar".to_string()].into_iter();
    //
    //     let result = parse(schema, content);
    //
    //     assert!(result.is_ok());
    //     let decoded = result.unwrap();
    //     assert_eq!(decoded.prefix, "expected_prefix");
    // }
    //
    #[test]
    fn test_schema_to_map() {
        let expected: HashMap<String, Vec<String>> = HashMap::from([
            ("A01".to_string(), vec!["A02".to_string()]),
            ("B01".to_string(), vec![]),
        ]); 

        let nodes = vec![SchemaNode {prefix: "A01".to_string(), model: HashMap::new(), children: vec![SchemaNode {prefix: "A02".to_string(), model: HashMap::new(), children: Vec::new()}]}, SchemaNode {prefix: "B01".to_string(), model: HashMap::new(), children: Vec::new()}];

        let result = schema_to_map(nodes);

        assert_eq!(result, expected);
    }
}
