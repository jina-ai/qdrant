#[path = "./jina_proto.rs"]
mod jina_proto;

use super::qdrant_types::{PyPayloadType, PySegmentConfig};
use crate::qdrant_types::PySegment;

use pyo3::prelude::*;
use pyo3::PyErr;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyException;
use segment::types::{PointIdType, VectorElementType, ScoredPoint, ScoreType, PayloadKeyType, TheMap, PayloadType};
use segment::entry::entry_point::{OperationResult, SegmentEntry, OperationError};
use segment::segment_constructor::segment_constructor::build_segment;
use prost::Message;
use prost_types::value;
use prost_types::Value;
use prost_types::Struct;
use std::io::Cursor;
use std::path::Path;
use numpy::PyArray1;
use serde_json;


fn handle_inner_result<T> (result: OperationResult<T>) -> PyResult<T> {
    match result {
        Err(error) => {
            match error {
                OperationError::WrongVector {expected_dim, received_dim} =>
                    Err(PyErr::new::<PyException, _>(
                        format!("Wrong vector. Expected_dim {} is different from received_dim {}",
                                expected_dim, received_dim))),
                OperationError::PointIdError {missed_point_id} =>
                    Err(PyErr::new::<PyException, _>(
                        format!("Wrong point id. Missed id {}",
                                missed_point_id))),
                OperationError::TypeError {field_name, expected_type} =>
                    Err(PyErr::new::<PyException, _>(
                        format!("Type Error. Field {} should be of type {}",
                                field_name, expected_type))),
                OperationError::ServiceError {description} =>
                    Err(PyErr::new::<PyException, _>(
                        format!("Service Error: {}",
                                description))),
            }
        },
        Ok(inner_result) => Ok(inner_result)
    }
}

#[pymethods]
impl PySegment {
    const DEFAULT_OP_NUM: u64 = u64::MAX; // Disable skip_by_version for now

    #[new]
    fn new(dir: String, config: &PySegmentConfig) -> Self {
        let segment_result = handle_inner_result(build_segment(Path::new(&dir), &config.config));
        segment_result.map(|segment| PySegment { segment }).unwrap()
    }

    pub fn index(&mut self, point_id: PointIdType, vector: &PyArray1<VectorElementType>) -> PyResult<bool> {
        let result = self.segment.upsert_point(PySegment::DEFAULT_OP_NUM, point_id, &vector.to_vec().unwrap());
        handle_inner_result(result)
    }

    pub fn set_full_payload(&mut self, point_id: PointIdType, payload: String) -> PyResult<bool> {
        let pypayloadtype: TheMap<PayloadKeyType, PyPayloadType> = serde_json::from_str(&payload).unwrap();
        // need to correct the points where a collection is expected and not a single value
        let inner_payload = pypayloadtype.into_iter().map(|(k, v)| {
            match v {
                PyPayloadType::Keyword(x) => (k, PayloadType::Keyword(vec![x])),
                PyPayloadType::Integer(x) => (k, PayloadType::Integer(vec![x])),
                PyPayloadType::Float(x) => (k, PayloadType::Float(vec![x])),
                PyPayloadType::KeywordVec(x) => (k, PayloadType::Keyword(x)),
                PyPayloadType::FloatVec(x) => (k, PayloadType::Float(x)),
                PyPayloadType::IntegerVec(x) => (k, PayloadType::Integer(x))
            }
        }).rev().collect();
        let result = self.segment.set_full_payload(PySegment::DEFAULT_OP_NUM, point_id, inner_payload);
        handle_inner_result(result)
    }

    pub fn set_full_payload_document(&mut self, point_id: PointIdType, payload: Vec<u8>) -> PyResult<bool> {
        fn _convert_doc_into_payload(doc: &jina_proto::DocumentProto) -> TheMap<PayloadKeyType, PayloadType> {
            let mut payload = TheMap::new();
            payload.insert("id".to_string(), PayloadType::Keyword(vec![doc.id.to_string()]));
            payload.insert("parent_id".to_string(), PayloadType::Keyword(vec![doc.parent_id.to_string()]));
            payload.insert("mime_type".to_string(), PayloadType::Keyword(vec![doc.mime_type.to_string()]));
            payload.insert("modality".to_string(), PayloadType::Keyword(vec![doc.modality.to_string()]));
            payload.insert("content_hash".to_string(), PayloadType::Keyword(vec![doc.content_hash.to_string()]));
            payload.insert("granularity".to_string(), PayloadType::Integer(vec![doc.granularity.into()]));
            payload.insert("adjacency".to_string(), PayloadType::Integer(vec![doc.adjacency.into()]));
            payload.insert("siblings".to_string(), PayloadType::Integer(vec![doc.siblings.into()]));
            payload.insert("offset".to_string(), PayloadType::Integer(vec![doc.offset.into()]));
            payload.insert("weight".to_string(), PayloadType::Float(vec![doc.weight.into()]));
            match &doc.content {
                None => (),
                Some(jina_proto::document_proto::Content::Buffer(_buffer)) => {
                    println!("Buffer content is not supported");
                    ()
                },
                Some(jina_proto::document_proto::Content::Blob(_blob)) => {
                    println!("Blob content is not supported");
                    ()
                },
                Some(jina_proto::document_proto::Content::Text(text)) =>  {
                    payload.insert("content".to_string(), PayloadType::Keyword(vec![text.to_string()]));
                    payload.insert("text".to_string(), PayloadType::Keyword(vec![text.to_string()]));
                    ()
                },
                Some(jina_proto::document_proto::Content::Uri(uri)) => {
                    payload.insert("content".to_string(), PayloadType::Keyword(vec![uri.to_string()]));
                    payload.insert("uri".to_string(), PayloadType::Keyword(vec![uri.to_string()]));
                    ()
                }
            }
            match &doc.tags {
                Some(tags) => {
                    for (k, v) in &tags.fields {
                        match &v.kind {
                            // TODO: problem, qdrant matching only works on integers
                            Some(value::Kind::NumberValue(x)) => payload.insert(k.to_string(), PayloadType::Integer(vec![x.clone() as i64])),
                            Some(value::Kind::StringValue(x)) => payload.insert(k.to_string(), PayloadType::Keyword(vec![x.to_string()])),
                            _ => None
                        };
                    }
                }
                None => ()
            };
            payload
        }
        let doc = jina_proto::DocumentProto::decode(&mut Cursor::new(payload)).unwrap();
        let inner_payload = _convert_doc_into_payload(&doc);
        let result = self.segment.set_full_payload(PySegment::DEFAULT_OP_NUM, point_id, inner_payload);
        handle_inner_result(result)
    }

    fn get_full_payload(&self, point_id: PointIdType) -> TheMap<PayloadKeyType, String> {
        let payload = self.segment.payload(point_id).unwrap();
        let mut results = TheMap::new();
        for (k, _v) in payload {
            match _v {
                PayloadType::Keyword(x) => results.insert(k, x.iter().map(|y| y.to_string()).collect()),
                PayloadType::Integer(x) => results.insert(k, x.iter().map(|y| y.to_string()).collect()),
                PayloadType::Float(x) => results.insert(k, x.iter().map(|y| y.to_string()).collect()),
                _ => None
            };
        }
        results
    }

    fn get_full_payload_as_document(&self, point_id: PointIdType) -> PyObject {
        //TODO: See how to better pass bytes without getting GIL: move all logic to new object.
        // Maybe create a PyDocument that wraps the conversion from Bytes and to Bytes and so on
        fn _get_string_value(value: &PayloadType) -> Option<String> {
            match value {
                PayloadType::Keyword(x) => Some(x.iter().map(|y| y.to_string()).collect()),
                _ => None
            }
        }

        fn _get_int_value(value: &PayloadType) -> Option<u32> {
            match value {
                PayloadType::Integer(x) => Some(x[0] as u32),
                _ => None
            }
        }

        fn _get_float_value(value: &PayloadType) -> Option<f32> {
            match value {
                PayloadType::Float(x) => Some(x[0] as f32),
                _ => None
            }
        }

        fn _set_document_text(doc: &mut jina_proto::DocumentProto, value: &PayloadType) {
            match _get_string_value(value) {
                Some(text) => {
                    doc.content = Some(jina_proto::document_proto::Content::Text(text))
                }
                None => ()
            }
        }

        fn _set_document_uri(doc: &mut jina_proto::DocumentProto, value: &PayloadType) {
            match _get_string_value(value) {
                Some(uri) => {
                    doc.content = Some(jina_proto::document_proto::Content::Uri(uri))
                }
                None => ()
            }
        }

        let payload = self.segment.payload(point_id).unwrap();
        let mut document = jina_proto::DocumentProto::default();
        let mut fields: Option<TheMap<String, Value>> = None;// TheMap::new(); //TheMap::new();
        for (k, v) in payload {
            match k.as_str() {
                "id" => _get_string_value(&v).map(|x| document.id = x).unwrap(),
                "mime_type" => _get_string_value(&v).map(|x| document.mime_type = x).unwrap(),
                "modality" => _get_string_value(&v).map(|x| document.modality = x).unwrap(),
                "parent_id" => _get_string_value(&v).map(|x| document.parent_id = x).unwrap(),
                "content_hash" => _get_string_value(&v).map(|x| document.content_hash = x).unwrap(),
                "granularity" => _get_int_value(&v).map(|x| document.granularity = x).unwrap(),
                "adjacency" => _get_int_value(&v).map(|x| document.adjacency = x).unwrap(),
                "siblings" => _get_int_value(&v).map(|x| document.siblings = x).unwrap(),
                "offset" => _get_int_value(&v).map(|x| document.offset = x).unwrap(),
                "weight" => _get_float_value(&v).map(|x| document.weight = x).unwrap(),
                "chunks" => (),
                "matches" => (),
                "evaluations" => (),
                "blob" => (),
                "buffer" => (),
                "content" => (),
                "text" => _set_document_text(&mut document, &v),
                "uri" => _set_document_uri(&mut document, &v),
                x => {
                    // set tag values
                    match v {
                        PayloadType::Float(y) => {
                            if let None = fields {
                                fields = Some(TheMap::new());
                            }
                            fields.as_mut().unwrap().insert(x.to_string(),Value{kind: Some(value::Kind::NumberValue(y[0] as f64))});
                        },
                        PayloadType::Integer(y) => {
                            if let None = fields {
                                fields = Some(TheMap::new());
                            }
                            fields.as_mut().unwrap().insert(x.to_string(),Value{kind: Some(value::Kind::NumberValue(y[0] as f64))});
                        },
                        PayloadType::Keyword(y) => {
                            if let None = fields {
                                fields = Some(TheMap::new());
                            }
                            fields.as_mut().unwrap().insert(x.to_string(), Value{kind: Some(value::Kind::StringValue(y[0].to_string()))});
                            ()
                        },
                        _ => ()
                    }
                }
            }
        }
        if let Some(f) = fields {
            document.tags = Some(Struct {fields: f});
        }
        let mut buf: Vec<u8> = Vec::with_capacity(document.encoded_len());
        document.encode(&mut buf).unwrap();
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyBytes::new(py, &buf).into()
    }

    pub fn delete(&mut self, point_id: PointIdType) -> PyResult<bool> {
        let result = self.segment.delete_point(PySegment::DEFAULT_OP_NUM, point_id);
        handle_inner_result(result)
    }

    pub fn search(&self, vector: &PyArray1<VectorElementType>, filter: Option<String>, top_k: usize) -> PyResult<(Vec<PointIdType>, Vec<ScoreType>)> {
        fn _convert_scored_point_vec(vec: Vec<ScoredPoint>) -> (Vec<PointIdType>, Vec<ScoreType>) {
            vec.into_iter().map(
                |scored_point| (scored_point.id, scored_point.score)).unzip()
        }
        let qdrant_filter = filter.map(|f| {
            serde_json::from_str(&f).unwrap()
        });
        let result = self.segment.search(&vector.to_vec().unwrap(), Option::from(&qdrant_filter), top_k, None);
        handle_inner_result(result.map(|vec| _convert_scored_point_vec(vec)))
    }
}
