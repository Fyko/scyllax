use std::fmt::Debug;

use scylla::{
    cql_to_rust::{FromCqlVal, FromCqlValError},
    frame::{
        response::result::{ColumnType, CqlValue},
        value::Value as ScyllaValue,
    },
    serialize::{value::SerializeCql, writers::WrittenCellProof, CellWriter, SerializationError},
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, PartialEq, Eq)]
pub struct Json<T: Debug + Clone + Serialize + DeserializeOwned>(pub T);

impl<T: Debug + Clone + Serialize + DeserializeOwned> std::fmt::Debug for Json<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Debug + Clone + Serialize + DeserializeOwned> ScyllaValue for Json<T> {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), scylla::frame::value::ValueTooBig> {
        let data = serde_json::to_string(&self.0).unwrap();
        <String as ScyllaValue>::serialize(&data, buf)
    }
}

impl<T: Debug + Clone + Serialize + DeserializeOwned> SerializeCql for Json<T> {
    fn serialize<'b>(
        &self,
        typ: &ColumnType,
        writer: CellWriter<'b>,
    ) -> Result<WrittenCellProof<'b>, SerializationError> {
        let data = serde_json::to_string(&self.0).unwrap();
        <String as SerializeCql>::serialize(&data, typ, writer)
    }
}

impl<T: Debug + Clone + Serialize + DeserializeOwned> FromCqlVal<CqlValue> for Json<T> {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        let data = <String as FromCqlVal<CqlValue>>::from_cql(cql_val)?;

        serde_json::from_str(&data)
            .map(Json)
            .ok()
            .ok_or(FromCqlValError::BadCqlType)
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        a: i32,
        b: String,
    }

    #[test]
    fn test_json_cql() {
        let value = Json(TestStruct {
            a: 1,
            b: "test".to_string(),
        });

        let mut buf: Vec<u8> = Vec::new();
        ScyllaValue::serialize(&value, &mut buf).unwrap();

        assert_eq!(
            buf,
            &[
                0, 0, 0, 18, 123, 34, 97, 34, 58, 49, 44, 34, 98, 34, 58, 34, 116, 101, 115, 116,
                34, 125
            ]
        );
    }
}
