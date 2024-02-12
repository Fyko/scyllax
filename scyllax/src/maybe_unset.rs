//! A wrapper around a value that can be unincluded but not overwritten/made null
use scylla::{
    _macro_internal::{Value, ValueTooBig},
    frame::{response::result::ColumnType, value::Counter},
    serialize::{value::SerializeCql, writers::WrittenCellProof, CellWriter, SerializationError},
    BufMut,
};

/// Represents an unset value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unset;

impl Value for Unset {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        // Unset serializes itself to empty value with length = -2
        buf.put_i32(-2);
        Ok(())
    }
}

/// A wrapper around a value that can be unincluded but not overwritten/made null
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum MaybeUnset<V> {
    /// The value is unset but shouldn't be overwritten
    #[default]
    Unset,
    /// The value is set
    Set(V),
}

impl<V: Value> Value for MaybeUnset<V> {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        match self {
            MaybeUnset::Set(v) => v.serialize(buf),
            MaybeUnset::Unset => Unset.serialize(buf),
        }
    }
}

impl<V: SerializeCql> SerializeCql for MaybeUnset<V> {
    fn serialize<'b>(
        &self,
        typ: &ColumnType,
        writer: CellWriter<'b>,
    ) -> Result<WrittenCellProof<'b>, SerializationError> {
        match self {
            MaybeUnset::Set(v) => v.serialize(typ, writer),
            MaybeUnset::Unset => Ok(writer.set_unset()),
        }
    }
}

// implement From<V> for MaybeUnset<V>
impl<V: Value> From<V> for MaybeUnset<V> {
    fn from(v: V) -> Self {
        MaybeUnset::Set(v)
    }
}

// implement From<Option<V>> for MaybeUnset<V>
impl<V: Value> From<Option<V>> for MaybeUnset<V> {
    fn from(v: Option<V>) -> Self {
        match v {
            Some(v) => MaybeUnset::Set(v),
            None => MaybeUnset::Unset,
        }
    }
}

// MaybeUnset<scylla_cql::frame::value::Counter>: From<i64>
impl From<i64> for MaybeUnset<Counter> {
    fn from(v: i64) -> Self {
        MaybeUnset::Set(Counter(v))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unset() {
        assert_eq!(MaybeUnset::<i32>::Unset, MaybeUnset::from(None));
        assert_eq!(MaybeUnset::<i32>::Set(1), MaybeUnset::from(Some(1)));

        assert_eq!(
            MaybeUnset::<&str>::Set("hello world"),
            MaybeUnset::from("hello world")
        );
        assert_eq!(MaybeUnset::<&str>::Unset, MaybeUnset::from(None::<&str>));

        assert_eq!(
            MaybeUnset::<String>::Set("hello world".to_string()),
            MaybeUnset::from("hello world".to_string())
        );
        assert_eq!(
            MaybeUnset::<String>::Unset,
            MaybeUnset::from(None::<String>)
        );

        assert_eq!(
            MaybeUnset::<Vec<u8>>::Set(vec![1, 2, 3]),
            MaybeUnset::from(vec![1, 2, 3])
        );
        assert_eq!(
            MaybeUnset::<Vec<u8>>::Unset,
            MaybeUnset::from(None::<Vec<u8>>)
        );

        assert_eq!(MaybeUnset::<bool>::Set(true), MaybeUnset::from(true));
        assert_eq!(MaybeUnset::<bool>::Unset, MaybeUnset::from(None::<bool>));

        let uuid = uuid::Uuid::new_v4();
        assert_eq!(MaybeUnset::<uuid::Uuid>::Set(uuid), MaybeUnset::from(uuid));
        assert_eq!(
            MaybeUnset::<uuid::Uuid>::Unset,
            MaybeUnset::from(None::<uuid::Uuid>)
        );
    }
}
