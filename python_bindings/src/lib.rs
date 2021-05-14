use pyo3::prelude::*;
use segment::types::{Distance, PointIdType, VectorElementType, ScoredPoint, ScoreType, Indexes, PayloadIndexType, StorageType, SegmentConfig};
use segment::segment::Segment;
use std::path::Path;
use segment::entry::entry_point::{OperationResult, SegmentEntry, OperationError};
use numpy::PyArray1;
use pyo3::PyErr;
use pyo3::exceptions::PyException;
use segment::segment_constructor::segment_constructor::build_segment;


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

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
struct PySegmentConfig {
    /// Size of a vectors used
    pub vector_size: usize,
    /// Type of index used for search
    pub index: Indexes,
    /// Payload Indexes
    pub payload_index: Option<PayloadIndexType>,
    /// Type of distance function used for measuring distance between vectors
    pub distance: Distance,
    /// Type of vector storage
    pub storage_type: StorageType,
}

#[pymethods]
impl PySegmentConfig {
    #[new]
    fn new(vector_size: usize, index: usize, payload_index: Option<usize>, distance: usize, storage_type: usize) -> Self {
        let ind = match index {
            0 => Some(Indexes::Plain {}),
            1 => Some(Indexes::Hnsw { m: 0, ef_construct: 0 }),
            _ => None
        };
        let pind = match payload_index {
            Some(0) => Some(PayloadIndexType::Plain),
            Some(1) => Some(PayloadIndexType::Struct),
            Some(x) => {
                println!("Invalid payload index type {}", x);
                None
            },
            None => None,
        };

        let d = match distance {
            0 => Some(Distance::Cosine),
            1 => Some(Distance::Dot),
            2 => Some(Distance::Euclid),
            _ => None
        };

        let stype = match storage_type {
            0 => Some(StorageType::InMemory),
            1 => Some(StorageType::Mmap),
            _ => None
        };


        PySegmentConfig { vector_size: vector_size,
            index: ind.unwrap(),
            payload_index: pind,
            distance: d.unwrap(),
            storage_type: stype.unwrap()}
    }
}


#[pyclass(unsendable, module="qdrant_segment_py", dict)]
struct PySegment {
    pub segment: Segment
}


#[pymethods]
impl PySegment {
    #[new]
    fn new(dir: String, config: &PySegmentConfig) -> Self {
        let segment_result = handle_inner_result(build_segment(Path::new(&dir), &SegmentConfig {
            vector_size: config.vector_size,
            index: config.index,
            payload_index: config.payload_index,
            distance: config.distance,
            storage_type: config.storage_type
        }));
        segment_result.map(|segment| PySegment{segment}).unwrap()
    }

    pub fn index(&mut self, point_id: PointIdType, vector: &PyArray1<VectorElementType>) -> PyResult<bool> {
        let result = self.segment.upsert_point(100, point_id, &vector.to_vec().unwrap());
        handle_inner_result(result)
    }

    pub fn search(&self, vector: &PyArray1<VectorElementType>, top_k: usize) -> PyResult<(Vec<PointIdType>, Vec<ScoreType>)> {
        fn _convert_scored_point_vec(vec: Vec<ScoredPoint>) -> (Vec<PointIdType>, Vec<ScoreType>) {
            vec.into_iter().map(
                |scored_point| (scored_point.id, scored_point.score)).unzip()
        }
        let result = self.segment.search(&vector.to_vec().unwrap(), None, top_k, None);
        handle_inner_result(result.map(|vec| _convert_scored_point_vec(vec)))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn qdrant_segment_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySegmentConfig>()?;
    m.add_class::<PySegment>()?;
    Ok(())
}
