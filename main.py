from flow_parser.flow_parser import process_dict

schema = [
    {
        "prefix": "A01",
        "model": {"foo": "string", "bar": "int"},
        "children": [
            {"prefix": "B01", "model": {"baz": "float"}, "children": []}
        ]
    },
]

parsed = process_dict(schema[0])
print(parsed)  # Will come back as Python objects (repr of Rust struct)

if __name__ == '__main__':
    ...