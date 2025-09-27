
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

print(result)

# > DecodedFlow(prefix="ROOT", contents=[], children=[
# >     DecodedFlow(prefix="A01", contents=["foo", "bar"], children=[
# >         DecodedFlow(prefix="A02", contents=["wiz", "bang"], children=[]),
# >     ]),
# >     DecodedFlow(prefix="B01", contents=["waz", "baz"], children=[]),
# > ])
