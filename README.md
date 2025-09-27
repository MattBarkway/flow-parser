## Flow Parser

This is a Rust library with optional Python bindings (via [pyo3](https://pyo3.rs/)) for parsing structured pipe-delimited files.

These files are used in a number of industries such as [healthcare](https://en.wikipedia.org/wiki/Health_Level_7#HL7_Version_2) and the [UK energy industry](https://www.electralink.co.uk/data-catalogues/dtc-catalogue/)

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
// > DecodedFlow(prefix="ROOT", contents=[], children=[
// >     DecodedFlow(prefix="A01", contents=["foo", "bar"], children=[
// >         DecodedFlow(prefix="A02", contents=["wiz", "bang"], children=[]),
// >     ]),
// >     DecodedFlow(prefix="B01", contents=["waz", "baz"], children=[]),
// > ])
```

### Contributing

Contributions are welcome.

Please open issues to report bug fixes, request features, or add documentation improvements.

### License

MIT

