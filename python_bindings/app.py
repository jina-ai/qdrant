import numpy as np
import json

from jina import Document, DocumentArray
from qdrant_segment_py import \
    PyVectorIndexType, \
    PyPayloadIndexType, \
    PyDistanceType, \
    PyStorageType, \
    PySegmentConfig, \
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

filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), TOP_K)
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
new_ids, new_scores = segment.search(query, None, 10)
assert set(new_ids) != ids

for id in new_ids:
    payload = segment.get_full_payload(id)
    extracted_doc = Document(payload)
    print(f' extracted_doc {extracted_doc}')
    assert extracted_doc.text == f'I am document {id}'


print(f' No we will add documents using the set_full_payload_document interface to serialize the document that will be loaded by the `DocumentProto` in rust')
new_docs = DocumentArray([Document(id=str(i), embedding=get_random_numpy(), text=f'I am document {i}', granularity=5, weight=5) for i in range(2000, 2010)])

for doc in new_docs:
    doc.tags['hello'] = 'world'
    doc.tags['inner_float'] = float(doc.id)
    result = segment.index(int(doc.id), doc.embedding)
    segment.set_full_payload_document(int(doc.id), doc.SerializeToString())
    payload = segment.get_full_payload(int(doc.id))
    extracted_doc = Document(payload)
    print(f' extracted_doc {extracted_doc}')
    assert extracted_doc.tags['hello'] == 'world'

filter = {}
field1 = {}
field1['key'] = 'hello'
field1['match'] = {'keyword': 'world'}
filter['should'] = [field1]

filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), 1000)
assert len(filtered_ids) == 10
assert len(filtered_scores) == 10

filter = {}
field1 = {}
field1['key'] = 'hello'
field1['match'] = {'keyword': 'world'}
field2 = {}
field2['key'] = 'inner_float'
field2['match'] = {'integer': 2005}
filter['should'] = [field1, field2]
print(f' filtered_ids {len(filtered_ids)}')
filtered_ids, filtered_scores = segment.search(query, json.dumps(filter), 1000)
print(f' len {len(filtered_ids)}')
assert len(filtered_ids) == 1
assert len(filtered_scores) == 1
assert filtered_ids[0] == 2005