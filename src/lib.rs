mod errors;
mod parser;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

// #[derive(Debug, Clone, Deserialize)]
pub struct SchemaNode {
    pub prefix: String,
    pub model: HashMap<String, FieldType>,
    pub children: Vec<SchemaNode>,
}

// #[derive(Debug, Deserialize, Clone)]
// #[serde(rename_all = "lowercase")]
#[derive(Clone)]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
}

// #[pyfunction]
// pub fn process_dict(map: Py<PyDict>, py: Python<'_>) -> PyResult<()> {
//     let map: SchemaNode = map.extract(py).unwrap();
//
//     if let Some(value) = map.get("apple") {
//         println!("Value for my 'apple': {}", value);
//     } else {
//         println!("Key not found");
//     }
//
//     Ok(())
// }

// #[pyfunction]
// fn load_schema(schema: Vec<SchemaNode>) -> PyResult<Vec<SchemaNode>> {
//     Ok(schema)
// }

// #[pymodule]
// fn flow_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(process_dict, m)?)?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_nesting() {}
}
