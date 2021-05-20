#[path = "./jina.rs"]
mod jina;

use super::qdrant_types::{PyPayloadType, PySegmentConfig};
use crate::qdrant_types::PySegment;

use pyo3::prelude::*;
use pyo3::PyErr;
use pyo3::exceptions::PyException;
use segment::types::{PointIdType, VectorElementType, ScoredPoint, ScoreType, PayloadKeyType, TheMap, PayloadType};
use segment::entry::entry_point::{OperationResult, SegmentEntry, OperationError};
use segment::segment_constructor::segment_constructor::build_segment;
use prost::Message;
use prost_types::value;
use std::io::Cursor;
use std::path::Path;
use numpy::PyArray1;
use serde_json;


fn handle_inner_result<T> (result: OperationResult<T>) -> PyResult<T> {
    match result {
        Err(error) => {
            match error {
                OperationError::WrongVector {expected_dim, received_dim} => Err(PyErr::new::<PyException, _>(format!("Wrong vector, expected_dim {} is different from received_dim {}", expected_dim, received_dim))),
                OperationError::PointIdError {missed_point_id} => Err(PyErr::new::<PyException, _>(format!("Wrong point id, Missed id {}", missed_point_id))),
                OperationError::TypeError {field_name, expected_type} => Err(PyErr::new::<PyException, _>(format!("Type Error, Field {} should be of type {}", field_name, expected_type))),
                OperationError::ServiceError {description} => Err(PyErr::new::<PyException, _>(format!("Service Error: {}", description))),
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
        fn _convert_doc_into_payload(doc: &jina::DocumentProto) -> TheMap<PayloadKeyType, PayloadType> {
            let mut payload = TheMap::new();
            payload.insert("id".to_string(), PayloadType::Keyword(vec![doc.id.to_string()]));
            payload.insert("granularity".to_string(), PayloadType::Integer(vec![doc.granularity.into()]));
            payload.insert("mime_type".to_string(), PayloadType::Keyword(vec![doc.mime_type.to_string()]));
            payload.insert("modality".to_string(), PayloadType::Keyword(vec![doc.modality.to_string()]));
            match &doc.tags {
                Some(tags) => {
                    for (k, v) in &tags.fields {
                        match &v.kind {
                            Some(value::Kind::NumberValue(x)) => payload.insert(k.to_string(), PayloadType::Float(vec![x.clone()])),
                            Some(value::Kind::StringValue(x)) => payload.insert(k.to_string(), PayloadType::Keyword(vec![x.to_string()])),
                            _ => None
                        };
                    }
                }
                None => ()
            };
            payload
        }
        let doc = jina::DocumentProto::decode(&mut Cursor::new(payload)).unwrap();
        let result = self.segment.set_full_payload(PySegment::DEFAULT_OP_NUM, point_id, _convert_doc_into_payload(&doc));
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