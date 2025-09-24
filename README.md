## Flow Parser

**Flow Parser** is a Rust library with optional Python bindings (via [pyo3](https://pyo3.rs/)) designed for parsing structured, pipe-delimited files, such as those commonly used in the energy industry. 

It provides a fast, reliable, and flexible way to process large flow files in both Rust and Python.

### Features

Generic parser for pipe-delimited (|) structured files

Optimized for large files typical in energy sector workflows

Easy-to-use Python API through pyo3 bindings

Fully written in Rust for performance and safety

### Installation

#### Python

Install via Poetry:
```bash
poetry add flow-parser
```

Usage:

```python
import flow_parser

schema = [
    {
        "prefix": "A01",
        "model": {"foo": "string", "bar": "int"},
        "children": [
            {"prefix": "B01", "model": {"baz": "float"}, "children": []}
        ]
    },
]

flow = """
A01|foo|bar|
B01|1.2|
A01|bing|bong|
...
""" 

flow_parser.parse(flow, schema=schema)

```

#### Rust

Install:
```bash
cargo add flow-parser
```

Usage:

```rust
let schema  = vec![
    SchemaNode {
        prefix: "A01",
        model: HashMap::new(),
        children: vec![SchemaNode {
            prefix: "A02",
            model: HashMap::new(),
            children: Vec::new(),
        }],
    },
    SchemaNode {
        prefix: "B01",
        model: HashMap::new(),
        children: Vec::new(),
    },
];
let content = vec!["A01|foo|bar|", "A02|wiz|bang|", "A01|bing|bong|", "B01|waz|baz|"];

let result = parse(schema, content.into_iter());

```

