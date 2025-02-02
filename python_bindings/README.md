# Python bindings for qdrant segment

This is the first iteration to try to build Python bindings for the segment module of Qdrant.

The intention is to offer the ANN functionality directly from Python without needing to have a Qdrant server.

## Instructions to build


### Prerequisites

#### Install rust, cargo and maturin

Check how to install [rust and cargo](https://www.rust-lang.org/tools/install) 

Install maturin as tool for building the project and wheels.

```shell
pip install maturin
```

#### Build the rust jina DocumentProto

Inside the subfolder `jina-proto`:

```shell
cargo build --release
```

#### Copy resulting `jina.rs` artifact

Inside the subfolder `src`:
```shell
cp ../jina-proto/target/release/build/jina-rust-proto-*/out/jina.rs jina_proto.rs
```

#### Build qdrant_segment_py

From this folder, run:

There may be some differences when running from macOS

```shell
maturin build --release --cargo-extra-args="-j 4"
pip install target/wheels/qdrant_segment_py*cp37*
```

In the case of macOS, try to run:
```shell
maturin build --release --cargo-extra-args="-j 4 --target x86_64-apple-darwin"
pip install target/wheels/qdrant_segment_py*cp37*
```

Then you can run:
```shell
python app.py
```

This is the content inside app.py:

```python
import numpy as np
import json

from jina import Document, DocumentArray
from qdrant_segment_py import
    PyVectorIndexType,
    PyPayloadIndexType,
    PyDistanceType,
    PyStorageType,
    PySegmentConfig,
    PySegment

TOP_K = 10
GRANULARITY_4_IDX_FILTER = 10

def get_random_numpy():
    return np.random.rand(100).astype('float32') # for now only accepts this type


vector_index_type = PyVectorIndexType(0)
payload_index_type = PyPayloadIndexType(0)
distance_type = PyDistanceType(0)
storage_type = PyStorageType(0)
vector_dim = 100

config = PySegmentConfig(vector_dim, vector_index_type, payload_index_type, distance_type, storage_type)
segment = PySegment('dir', config)

docs = DocumentArray([Document(id=str(i), embedding=get_random_numpy(), text=f'I am document {i}', granularity=5, weight=5) for i in range(1000)])
docs[GRANULARITY_4_IDX_FILTER].granularity = 4

for i, doc in enumerate(docs):
    result = segment.index(int(doc.id), doc.embedding)
    payload = doc.dict()
    # cannot handle embedding type properly
    del payload['embedding']
    segment.set_full_payload(int(doc.id), json.dumps(payload))

query = get_random_numpy()

print(f' First search (No filter): Expect to retrieve 10 documents')
ids, scores = segment.search(query, None, TOP_K)
assert len(ids) == TOP_K
assert len(scores) == TOP_K

for id in ids:
    payload = segment.get_full_payload(id)
    extracted_doc = Document(payload)
    print(f' extracted_doc {extracted_doc}')
    assert extracted_doc.text == f'I am document {id}'
    segment.delete(id)

print(f' Second search (Filter granularity 4): Expect to retrieve 1 documents')

filter = {}
field = {}
field['key'] = 'granularity'
field['match'] = {'integer': 4}
filter['should'] = [field]

filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), TOP_K, None)
assert len(filtered_ids) == 1
assert len(filtered_scores) == 1
assert filtered_ids[0] == GRANULARITY_4_IDX_FILTER

for id in filtered_ids:
    payload = segment.get_full_payload(id)
    extracted_doc = Document(payload)
    print(f' extracted_doc {extracted_doc}')
    assert extracted_doc.text == f'I am document {id}'

print(f' Remove first set of results')

deleted_ids = []
for id in ids:
    deleted_ids.append(id)
    segment.delete(id)

print(f' Third search (No filter): Expect 10 results but different from the first since they were removed')
new_ids, new_scores = segment.search(query, None, 10, None)
assert set(new_ids) != ids

for id in new_ids:
    payload = segment.get_full_payload(id)
    extracted_doc = Document(payload)
    print(f' extracted_doc {extracted_doc}')
    assert extracted_doc.text == f'I am document {id}'


print(f' No we will add documents using the set_full_payload_document interface to serialize the document that will be loaded by the `DocumentProto` in rust '
      f' \n Only one of them does not have `tags["hello"] = "world". Inside qdrant. "tags" content is flattened because qdrant does not support nested payload. And '
      f'these fields would not be filterable')
new_docs = DocumentArray([Document(id=str(i), embedding=get_random_numpy(), text=f'I am document {i}', granularity=5, weight=5) for i in range(2000, 2010)])

for doc in new_docs:
    if int(doc.id) != 2005:
        doc.tags['hello'] = 'world'
    doc.tags['_id'] = float(doc.id)
    result = segment.index(int(doc.id), doc.embedding)
    segment.set_full_payload_document(int(doc.id), doc.SerializeToString())
    payload = segment.get_full_payload_as_document(int(doc.id))
    extracted_doc = Document(payload)
    if int(extracted_doc.id) != 2005:
        assert extracted_doc.tags['hello'] == 'world'
    print(f' extracted_doc from rust DocumentProto {Document(payload)}')

filter = {}
field = {}
field['key'] = 'hello'
field['match'] = {'keyword': 'world'}
filter['should'] = [field]

print(f' Now we do search with a filter that should only match new added Documents with tags["hello"] = "world" \n'
      f'=> {filter}')
filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), 1000, None)
assert len(filtered_ids) == 9
assert len(filtered_scores) == 9
print(f' RESULT {filtered_ids}')

filter = {}
field = {}
field['key'] = '_id'
field['match'] = {'integer': 2005}
filter['should'] = [field]
print(f' Now we do search with a filter that should only match new added Documents with tags["_id"] = 2005\n'
      f'=> {filter}')
filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), 1000, None)
assert len(filtered_ids) == 1
assert len(filtered_scores) == 1
assert set(filtered_ids) == {2005}
print(f' RESULT {filtered_ids}')


filter = {}
field1 = {}
field1['key'] = 'hello'
field1['match'] = {'keyword': 'world'}
field2 = {}
inner_field1 = {}
inner_field1['key'] = '_id'
inner_field1['match'] = {'integer': 2005}
inner_field2 = {}
inner_field2['key'] = '_id'
inner_field2['match'] = {'integer': 2006}
field2['should'] = [inner_field1, inner_field2]
filter['must'] = [field1, field2]
print(f' Now we do search with a filter that should only match new added Documents with tags["_id"] = 2005 or 2006 and tags["hello"] == "world"\n'
      f'=> {filter}')

filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), 1000, None)
assert len(filtered_ids) == 1
assert len(filtered_scores) == 1
assert set(filtered_ids) == {2006}
print(f' RESULT {filtered_ids}')
```

Limitations for Jina.

`PointIdType` is currently of `u64`.

# TODO
- Add tests with `python`
- Expose more advanced options from `segment` module
- Understand how to do `setuptools` to distribute
