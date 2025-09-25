use crate::errors::FlowParseError;
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::models::{DecodedFlow, Node, SchemaNode};

fn build_children_map(node: SchemaNode, map: &mut HashMap<String, Vec<String>>) {
    let child_prefixes = node.children.iter().map(|c| c.prefix.clone()).collect();
    map.insert(node.prefix.clone(), child_prefixes);

    for child in node.children {
        build_children_map(child, map);
    }
}

fn wrap_with_root(schema: Vec<SchemaNode>) -> SchemaNode {
    SchemaNode {
        prefix: "ROOT".to_string(),
        model: HashMap::new(),
        children: schema,
    }
}

pub fn load_schema(path: &str) -> Result<Vec<SchemaNode>, FlowParseError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).map_err(|e| FlowParseError::Serde(e))?)
}

fn separate_prefix<'a>(line: &'a str) -> Result<(&'a str, Vec<&'a str>), FlowParseError> {
    let line = line.trim_end_matches('|');
    let mut split = line.splitn(2, '|');
    let prefix = split
        .next()
        .ok_or(FlowParseError::Invalid("Missing prefix".to_owned()))?;
    let contents: Vec<&'a str> = split
        .next()
        .map(|rest| rest.split('|').collect())
        .ok_or(FlowParseError::Invalid("Missing contents".to_owned()))?;
    Ok((prefix, contents))
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

pub fn parse<'a, I>(schema: Vec<SchemaNode>, mut content: I) -> Result<DecodedFlow, FlowParseError>
where
    I: Iterator<Item = &'a str>,
{
    let flattened_schema = wrap_with_root(schema);
    let mut nodes = Vec::new();
    nodes.push(Node {
        prefix: "ROOT".to_string(),
        contents: vec![],
        children: vec![],
    });

    let mut stack = vec![0];
    let schema_map = &mut HashMap::new();
    build_children_map(flattened_schema, schema_map);
    while let Some(row) = content.next() {
        let (prefix, contents) = separate_prefix(row)?;
        let node = Node {
            prefix: prefix.to_owned(),
            contents: contents.into_iter().map(String::from).collect(),
            children: Vec::new(),
        };
        let idx = nodes.len();
        nodes.push(node);

        // TODO only add to stack is self.children not empty
        while let Some(&candidate_parent_idx) = stack.last() {
            let stack_top = &nodes[candidate_parent_idx];
            if stack_top.prefix == "ROOT" {
                // ROOT node always counts as valid parent
                // Only ever present in stack on first row
                nodes[candidate_parent_idx].children.push(idx);

                let has_children = schema_map
                    .get(prefix)
                    .ok_or(FlowParseError::Invalid("Unknown prefix".to_string()))?
                    .len()
                    > 0;
                if has_children {
                    stack.push(idx);
                }
                break;
            } else {
                let stack_top_schema = schema_map
                    .get(&stack_top.prefix)
                    .ok_or(FlowParseError::Invalid("Unknown prefix".to_string()))?;
                // Check if stack top is parent of current row
                if stack_top_schema.contains(&prefix.to_string()) {
                    // If stack top is valid parent, add idx to stack
                    nodes[candidate_parent_idx].children.push(idx);
                    let has_children = schema_map
                        .get(prefix)
                        .ok_or(FlowParseError::Invalid("Unknown prefix".to_string()))?
                        .len()
                        > 0;
                    if has_children {
                        stack.push(idx);
                    }
                    break;
                } else {
                    // stack top is not valid parent, pop stack
                    stack.pop();
                    continue;
                }
            }
        }
    }

    Ok(build_nested(&nodes, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        // Setup test data
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
