## Flow Parser

**Flow Parser** is a Rust library with optional Python bindings (via [pyo3](https://pyo3.rs/)) designed for parsing structured, pipe-delimited files, such as those commonly used in the energy industry. 

It provides a fast, reliable, and flexible way to process large flow files in both Rust and Python.


### Installation

#### Python

Install via Poetry:
```bash
poetry add flow-parser
```

Usage:

```python
from py_flow_parser import SchemaNode, FieldType, DecodedFlow, parse

schema =[
    SchemaNode(
        prefix="A01",
        model={},
        children=[
            SchemaNode(
                prefix="A02",
                model={},
                children=[]
            )]),
    SchemaNode(
        prefix="B01",
        model={},
        children=[]
    )
]
content = [
    "A01|foo|bar|",
    "A02|wiz|bang|",
    "A01|bing|bong|",
    "B01|waz|baz|",
]

result = parse(schema, content)
# > DecodedFlow(prefix="ROOT", contents=[], children=[
# >     DecodedFlow(prefix="A01", contents=["foo", "bar"], children=[
# >         DecodedFlow(prefix="A02", contents=["wiz", "bang"], children=[]),
# >     ]),
# >     DecodedFlow(prefix="B01", contents=["waz", "baz"], children=[]),
# > ])
```

#### Rust

Install:
```bash
cargo add flow_parser
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
# > DecodedFlow(prefix="ROOT", contents=[], children=[
# >     DecodedFlow(prefix="A01", contents=["foo", "bar"], children=[
# >         DecodedFlow(prefix="A02", contents=["wiz", "bang"], children=[]),
# >     ]),
# >     DecodedFlow(prefix="B01", contents=["waz", "baz"], children=[]),
# > ])
```

### Contributing

Contributions are welcome!

Please open issues or pull requests for bug fixes, features, or documentation improvements.

### License

MIT

