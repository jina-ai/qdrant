
use crate::types::{PointOffsetType, PayloadKeyType, PayloadType, Filter, TheMap, PayloadSchemaType};
use crate::entry::entry_point::OperationResult;
use schemars::_serde_json::Value;

/// Trait for payload data storage. Should allow filter checks
pub trait PayloadStorage {

    fn _extract_payloads<I>(_payload: I, prefix_key: Option<PayloadKeyType>) -> Vec<(PayloadKeyType, Option<PayloadType>)>
        where I: Iterator<Item=(PayloadKeyType, serde_json::value::Value)> {
        _payload.flat_map(|(k, value)| {
            let key = match &prefix_key {
                None => k,
                Some(_k) => _k.to_owned() + "__" + &k,
            };
            match value {
                Value::Null => vec![(key, None)],
                Value::Bool(x) => vec![(key, Some(PayloadType::Keyword(vec![x.to_string()])))],
                Value::Number(x) => vec![(key, Some(PayloadType::Integer(vec![x.as_i64().unwrap()])))],
                Value::String(x) => vec![(key, Some(PayloadType::Keyword(vec![x.to_string()])))],
                Value::Array(_) => vec![],
                Value::Object(x) => {
                    _extract_payloads(x.into_iter(), Some(key))
                }
            }
        } ).collect()
    }

    fn assign_all_with_value(&mut self, point_id: PointOffsetType, payload: TheMap<PayloadKeyType, serde_json::value::Value>) -> OperationResult<()> {
        self.drop(point_id)?;
        let inner_payloads = self._extract_payloads(payload.into_iter(), None);
        for (key, value) in inner_payloads {
            match value {
                Some(v) => self.assign(point_id, &key, v)?,
                None => (),
            }
        }
        Ok(())
    }

    /// Assign same payload to each given point
    fn assign_all(&mut self, point_id: PointOffsetType, payload: TheMap<PayloadKeyType, PayloadType>) -> OperationResult<()> {
        self.drop(point_id)?;
        for (key, value) in payload {
            self.assign(point_id, &key, value)?;
        }

        Ok(())
    }

    /// Assign payload to a concrete point with a concrete payload value
    fn assign(&mut self, point_id: PointOffsetType, key: &PayloadKeyType, payload: PayloadType) -> OperationResult<()>;

    /// Get payload for point
    fn payload(&self, point_id: PointOffsetType) -> TheMap<PayloadKeyType, PayloadType>;

    /// Delete payload by key
    fn delete(&mut self, point_id: PointOffsetType, key: &PayloadKeyType) -> OperationResult<Option<PayloadType>>;

    /// Drop all payload of the point
    fn drop(&mut self, point_id: PointOffsetType) -> OperationResult<Option<TheMap<PayloadKeyType, PayloadType>>>;

    /// Completely drop payload. Pufff!
    fn wipe(&mut self) -> OperationResult<()>;

    /// Force persistence of current storage state.
    fn flush(&self) -> OperationResult<()>;

    /// Get payload schema, automatically generated from payload
    fn schema(&self) -> TheMap<PayloadKeyType, PayloadSchemaType>;

    /// Iterate all point ids with payload
    fn iter_ids(&self) -> Box<dyn Iterator<Item=PointOffsetType> + '_>;
}


pub trait ConditionChecker {
    /// Check if point satisfies filter condition
    fn check(&self, point_id: PointOffsetType, query: &Filter) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn test_extract_payload_from_serde_json() {
    //     let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "metadata": {
    //             "height": 50,
    //             "width": 60
    //         }
    //     }"#;
    //
    //     let v: Value = serde_json::from_str(data)?;
    //     let a = Map
    //     let payload = PayloadStorage::_extract_payloads(v);
    // }

}