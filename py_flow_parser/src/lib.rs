use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use flow_parser::models::{DecodedFlow as RsDecodedFlow, FieldType as RsFieldType, SchemaNode as RsSchemaNode};
use flow_parser::parser::parse as flow_parse;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SchemaNode {
    #[pyo3(get, set)]
    pub prefix: String,

    #[pyo3(get, set)]
    pub model: HashMap<String, FieldType>,

    #[pyo3(get, set)]
    pub children: Vec<SchemaNode>,
}

impl From<SchemaNode> for RsSchemaNode {
    fn from(py: SchemaNode) -> Self {
        RsSchemaNode {
            prefix: py.prefix,
            model: py.model.into_iter().map(|(k, v)| (k, v.into())).collect(),
            children: py.children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[pymethods]
impl SchemaNode {
    #[new]
    fn new(
        prefix: String,
        model: HashMap<String, FieldType>,
        children: Vec<SchemaNode>,
    ) -> Self {
        SchemaNode {
            prefix,
            model,
            children,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "SchemaNode(prefix='{}', model={:?}, children={})",
            self.prefix,
            self.model,
            self.children.len()
        )
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Int,
    Float,
    Bool,
}

impl From<FieldType> for RsFieldType {
    fn from(py_ft: FieldType) -> Self {
        match py_ft {
            FieldType::String => RsFieldType::String,
            FieldType::Int => RsFieldType::Int,
            FieldType::Float => RsFieldType::Float,
            FieldType::Bool => RsFieldType::Bool,
        }
    }
}

impl From<RsFieldType> for FieldType {
    fn from(ft: RsFieldType) -> Self {
        match ft {
            RsFieldType::String => FieldType::String,
            RsFieldType::Int => FieldType::Int,
            RsFieldType::Float => FieldType::Float,
            RsFieldType::Bool => FieldType::Bool,
        }
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct DecodedFlow {
    #[pyo3(get)]
    pub prefix: String,
    #[pyo3(get)]
    pub contents: Vec<String>,
    #[pyo3(get)]
    pub children: Vec<DecodedFlow>,
}

impl From<RsDecodedFlow> for DecodedFlow {
    fn from(df: RsDecodedFlow) -> Self {
        DecodedFlow {
            prefix: df.prefix,
            contents: df.contents,
            children: df.children.into_iter().map(DecodedFlow::from).collect(),
        }
    }
}

#[pymethods]
impl DecodedFlow {
    #[new]
    fn new(
        prefix: String,
        contents: Vec<String>,
        children: Vec<DecodedFlow>,
    ) -> Self {
        DecodedFlow {
            prefix,
            contents,
            children,
        }
    }


    fn __repr__(&self) -> String {
        format!(
            "DecodedFlow(prefix='{}', contents={:?}, children={})",
            self.prefix,
            self.contents,
            self.children.len()
        )
    }
}

#[pyfunction]
pub fn parse(schema: Vec<SchemaNode>, content: Vec<String>) -> PyResult<DecodedFlow> {
    let rust_schema: Vec<RsSchemaNode> = schema.into_iter().map(|s| s.into()).collect();

    match flow_parse(rust_schema, content.iter().map(|s| s.as_str())) {
        Ok(decoded) => Ok(DecodedFlow::from(decoded)),
        Err(e) => Err(PyValueError::new_err(format!("Parse error: {:?}", e))),
    }
}

#[pymodule]
fn py_flow_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FieldType>()?;
    m.add_class::<SchemaNode>()?;
    m.add_class::<DecodedFlow>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}


