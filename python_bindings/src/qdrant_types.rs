use pyo3::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};
use segment::types::{Distance, Indexes, PayloadIndexType, StorageType, SegmentConfig, IntPayloadType, FloatPayloadType, SearchParams};
use segment::segment::Segment;

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PyVectorIndexType {
    index: Indexes
}

#[pymethods]
impl PyVectorIndexType {
    #[new]
    fn new(index_type: usize, m: Option<usize>, ef: Option<usize>) -> Self {
        let ind = match index_type {
            0 => Some(Indexes::Plain {}),
            1 => Some(Indexes::Hnsw { m: m.unwrap(), ef_construct: ef.unwrap()}),
            x => {
                println!("Invalid vector index type {}", x);
                None
            },
        };

        PyVectorIndexType { index: ind.unwrap() }
    }
}

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PyPayloadIndexType {
    payload_index_type: PayloadIndexType
}

#[pymethods]
impl PyPayloadIndexType {
    //TODO: LEARN HOW TO ALSO ENABLE READING FROM TEXT
    #[new]
    fn new(payload_index: usize) -> Self {
        let pind = match payload_index {
            0 => Some(PayloadIndexType::Plain {}),
            1 => Some(PayloadIndexType::Struct {}),
            x => {
                println!("Invalid payload index type {}", x);
                None
            },
        };
        PyPayloadIndexType { payload_index_type: pind.unwrap()}
    }
}

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PyDistanceType {
    distance: Distance
}

#[pymethods]
impl PyDistanceType {
    //TODO: LEARN HOW TO ALSO ENABLE READING FROM TEXT
    #[new]
    fn new(distance: usize) -> Self {
        let d = match distance {
            0 => Some(Distance::Cosine),
            1 => Some(Distance::Dot),
            2 => Some(Distance::Euclid),
            x => {
                println!("Invalid distance type {}", x);
                None
            },
        };
        PyDistanceType { distance: d.unwrap()}
    }
}

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PyStorageType {
    storage: StorageType
}

#[pymethods]
impl PyStorageType {
    //TODO: LEARN HOW TO ALSO ENABLE READING FROM TEXT
    #[new]
    fn new(storage: usize) -> Self {
        let stype = match storage {
            0 => Some(StorageType::InMemory),
            1 => Some(StorageType::Mmap),
            x => {
                println!("Invalid storage type {}", x);
                None
            },
        };
        PyStorageType { storage: stype.unwrap()}
    }
}


#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PySearchParams {
    pub params: SearchParams
}

#[pymethods]
impl PySearchParams {
    //TODO: LEARN HOW TO ALSO ENABLE READING FROM TEXT
    #[new]
    fn new(ef: usize) -> Self {
        PySearchParams { params: SearchParams::Hnsw {ef}}
    }
}

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PySegmentConfig {
    pub config: SegmentConfig
}

#[pymethods]
impl PySegmentConfig {
    #[new]
    fn new(vector_size: usize,
           index: &PyVectorIndexType,
           payload_index: Option<&PyPayloadIndexType>,
           distance: &PyDistanceType,
           storage_type: &PyStorageType) -> Self {

        let config = SegmentConfig { vector_size,
            index: index.index,
            payload_index: payload_index.map(|pid| pid.payload_index_type),
            distance: distance.distance,
            storage_type: storage_type.storage};

        PySegmentConfig { config }
    }

    #[staticmethod]
    fn from_config_file(dir: String) ->  Self {
        let file = File::open(Path::new(&dir));
        match file {
            Ok(f) => PySegmentConfig {config: serde_json::from_reader( BufReader::new(f)).unwrap() },
            _ => PySegmentConfig { config: SegmentConfig {
                vector_size: 0,
                index: Default::default(),
                payload_index: None,
                distance: Distance::Cosine,
                storage_type: Default::default()
            } }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, FromPyObject)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum PyPayloadType {
    Keyword(String),
    Integer(IntPayloadType),
    Float(FloatPayloadType),
    KeywordVec(Vec<String>),
    IntegerVec(Vec<IntPayloadType>),
    FloatVec(Vec<FloatPayloadType>)
}

#[pyclass(unsendable, module="qdrant_segment_py", dict)]
pub struct PySegment {
    pub segment: Segment
}
