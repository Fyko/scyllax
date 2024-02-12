use scylla::cql_to_rust::{FromCqlVal, FromCqlValError};
use scylla::frame::response::result::CqlValue;
use serde::{Deserialize, Serialize};

/// An implementation of a JSON type for ScyllaDB.
///
/// Also implements `From<Json>` for `prost_types::Struct` and vice versa.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonBlob(pub serde_json::Map<String, serde_json::Value>);

impl std::fmt::Debug for JsonBlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl scylla::frame::value::Value for JsonBlob {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), scylla::frame::value::ValueTooBig> {
        let data = serde_json::to_vec(&self.0).unwrap();
        <Vec<u8> as scylla::frame::value::Value>::serialize(&data, buf)
    }
}

impl FromCqlVal<CqlValue> for JsonBlob {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        let data = <String as FromCqlVal<CqlValue>>::from_cql(cql_val)?;

        serde_json::from_str(&data)
            .map(JsonBlob)
            .ok()
            .ok_or(FromCqlValError::BadCqlType)
    }
}

#[cfg(feature = "grpc")]
impl From<JsonBlob> for prost_types::Struct {
    fn from(json: JsonBlob) -> Self {
        fn to_struct(json: serde_json::Map<String, serde_json::Value>) -> prost_types::Struct {
            prost_types::Struct {
                fields: json
                    .into_iter()
                    .map(|(k, v)| (k, serde_json_to_prost(v)))
                    .collect(),
            }
        }

        fn serde_json_to_prost(json: serde_json::Value) -> prost_types::Value {
            use prost_types::value::Kind;
            use serde_json::Value;

            prost_types::Value {
                kind: Some(match json {
                    Value::Null => Kind::NullValue(0 /* wot? */),
                    Value::Bool(v) => Kind::BoolValue(v),
                    Value::Number(n) => {
                        Kind::NumberValue(n.as_f64().expect("Non-f64-representable number"))
                    }
                    Value::String(s) => Kind::StringValue(s),
                    Value::Array(v) => Kind::ListValue(prost_types::ListValue {
                        values: v.into_iter().map(serde_json_to_prost).collect(),
                    }),
                    Value::Object(v) => Kind::StructValue(to_struct(v)),
                }),
            }
        }

        to_struct(json.0)
    }
}

#[cfg(feature = "grpc")]
impl From<prost_types::Struct> for JsonBlob {
    fn from(value: prost_types::Struct) -> Self {
        fn from_struct(struct_: prost_types::Struct) -> serde_json::Map<String, serde_json::Value> {
            struct_
                .fields
                .into_iter()
                .map(|(k, v)| (k, prost_to_serde_json(v)))
                .collect()
        }

        fn prost_to_serde_json(value: prost_types::Value) -> serde_json::Value {
            use prost_types::value::Kind;
            use serde_json::Value;

            match value.kind.unwrap() {
                Kind::NullValue(_) => Value::Null,
                Kind::BoolValue(v) => Value::Bool(v),
                Kind::NumberValue(n) => Value::Number(
                    serde_json::Number::from_f64(n).expect("Non-f64-representable number"),
                ),
                Kind::StringValue(s) => Value::String(s),
                Kind::ListValue(v) => {
                    Value::Array(v.values.into_iter().map(prost_to_serde_json).collect())
                }
                Kind::StructValue(v) => Value::Object(from_struct(v)),
            }
        }

        JsonBlob(from_struct(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_map() -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        map.insert("a".to_string(), json!(1));
        map.insert("b".to_string(), json!("2"));
        map.insert("c".to_string(), json!([1, 2, 3]));
        map.insert("d".to_string(), json!({"e": "f"}));
        map
    }

    #[test]
    fn test_json() {
        let json = JsonBlob(create_map());
        let data = serde_json::to_string(&json).unwrap();
        assert_eq!(data, r#"{"a":1,"b":"2","c":[1,2,3],"d":{"e":"f"}}"#);
    }

    #[test]
    fn test_json_from_cql() {
        let json = JsonBlob(create_map());
        let cql_val = CqlValue::Text(serde_json::to_string(&json).unwrap());
        let json2 = JsonBlob::from_cql(cql_val).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_json_from_cql_bad_type() {
        let cql_val = CqlValue::Int(1);
        let json = JsonBlob::from_cql(cql_val);
        assert!(json.is_err());
    }

    #[test]
    fn test_json_from_cql_bad_json() {
        let cql_val = CqlValue::Text("bad json".to_string());
        let json = JsonBlob::from_cql(cql_val);
        assert!(json.is_err());
    }
}
