mod qdrant_types;
mod qdrant_segment;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn qdrant_segment_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<qdrant_types::PySearchParams>()?;
    m.add_class::<qdrant_types::PyVectorIndexType>()?;
    m.add_class::<qdrant_types::PyPayloadIndexType>()?;
    m.add_class::<qdrant_types::PyDistanceType>()?;
    m.add_class::<qdrant_types::PyStorageType>()?;
    m.add_class::<qdrant_types::PySegmentConfig>()?;
    m.add_class::<qdrant_types::PySegment>()?;
    Ok(())
}
