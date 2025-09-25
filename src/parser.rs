use crate::errors::FlowParseError;
use crate::models::{DecodedFlow, Node, SchemaNode, NodeArena};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn build_children_map(node: SchemaNode, map: &mut HashMap<String, Vec<String>>) {
    let child_prefixes = node.children.iter().map(|c| c.prefix.clone()).collect();
    map.insert(node.prefix.clone(), child_prefixes);

    for child in node.children {
        build_children_map(child, map);
    }
}

fn wrap_with_root(schema: Vec<SchemaNode>) -> SchemaNode {
    SchemaNode::new("ROOT", HashMap::new(), schema)
}

pub fn load_schema(path: &str) -> Result<Vec<SchemaNode>, FlowParseError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).map_err(|e| FlowParseError::Serde(e))?)
}

fn separate_prefix(line: &str) -> Result<(&str, Vec<&str>), FlowParseError> {
    let line = line.trim_end_matches('|');
    let mut parts = line.splitn(2, '|');
    match (parts.next(), parts.next()) {
        (Some(prefix), Some(rest)) => Ok((prefix, rest.split('|').collect())),
        _ => Err(FlowParseError::Invalid(
            "Missing prefix or contents".to_owned(),
        )),
    }
}

fn build_nested(nodes: &[Node], idx: usize) -> DecodedFlow {
    let node = &nodes[idx];
    DecodedFlow {
        prefix: node.prefix.clone(),
        contents: node.contents.clone(),
        children: node
            .children
            .iter()
            .map(|&child_idx| build_nested(nodes, child_idx))
            .collect(),
    }
}

fn is_valid_parent(
    prefix: &str,
    parent_prefix: &str,
    schema_map: &HashMap<String, Vec<String>>,
) -> Result<bool, FlowParseError> {
    Ok(parent_prefix == "ROOT"
        || schema_map
            .get(parent_prefix)
            .ok_or_else(|| FlowParseError::Invalid(format!("Unknown prefix: {parent_prefix}")))?
            .contains(&prefix.to_string()))
}

fn has_children(
    prefix: &str,
    schema_map: &HashMap<String, Vec<String>>,
) -> Result<bool, FlowParseError> {
    Ok(schema_map
        .get(prefix)
        .ok_or_else(|| FlowParseError::Invalid(format!("Unknown prefix: {prefix}")))?
        .len()
        > 0)
}

fn attach_to_parent(
    nodes: &mut Vec<Node>,
    stack: &mut Vec<usize>,
    idx: usize,
    prefix: &str,
    schema_map: &HashMap<String, Vec<String>>,
) -> Result<(), FlowParseError> {
    while let Some(&parent_idx) = stack.last() {
        if is_valid_parent(prefix, &nodes[parent_idx].prefix, schema_map)? {
            nodes[parent_idx].children.push(idx);
            if has_children(prefix, schema_map)? {
                stack.push(idx);
            }
            break;
        } else {
            stack.pop();
        }
    }
    Ok(())
}

pub fn parse<'a, I>(schema: Vec<SchemaNode>, content: I) -> Result<DecodedFlow, FlowParseError>
where
    I: Iterator<Item = &'a str>,
{
    let mut nodes = vec![Node::new("ROOT", vec![])];
    let mut stack = vec![0];
    let mut schema_map = HashMap::new();
    build_children_map(wrap_with_root(schema), &mut schema_map);

    for row in content {
        let (prefix, contents) = separate_prefix(row)?;
        let idx = nodes.push_node(prefix, contents);
        attach_to_parent(&mut nodes, &mut stack, idx, prefix, &schema_map)?;
    }

    Ok(build_nested(&nodes, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let schema = vec![
            SchemaNode {
                prefix: "A01".to_string(),
                model: HashMap::new(),
                children: vec![SchemaNode {
                    prefix: "A02".to_string(),
                    model: HashMap::new(),
                    children: Vec::new(),
                }],
            },
            SchemaNode {
                prefix: "B01".to_string(),
                model: HashMap::new(),
                children: Vec::new(),
            },
        ];
        let content = vec![
            "A01|foo|bar|",
            "A02|wiz|bang|",
            "A01|bing|bong|",
            "B01|waz|baz|",
        ]
        .into_iter();

        let result = parse(schema, content);

        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded.prefix, "ROOT");
        assert_eq!(decoded.children.len(), 3);
        assert_eq!(
            decoded.children[0],
            DecodedFlow {
                prefix: "A01".to_string(),
                contents: vec!["foo".to_string(), "bar".to_string()],
                children: vec![DecodedFlow {
                    prefix: "A02".to_string(),
                    contents: vec!["wiz".to_string(), "bang".to_string()],
                    children: vec![]
                }]
            }
        );
    }
}
