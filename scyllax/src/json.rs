use std::fmt::Debug;

use scylla::{
    cql_to_rust::{FromCqlVal, FromCqlValError},
    frame::{response::result::CqlValue, value::Value as ScyllaValue},
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
        let data = serde_json::to_vec(&self.0).unwrap();
        <Vec<u8> as ScyllaValue>::serialize(&data, buf)
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
