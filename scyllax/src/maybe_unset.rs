//! A wrapper around a value that can be unincluded but not overwritten/made null
use scylla::{
    _macro_internal::{Value, ValueTooBig},
    frame::value::Unset,
};

/// A wrapper around a value that can be unincluded but not overwritten/made null
#[derive(Clone, Copy, Debug)]
pub enum MaybeUnset<V: Value> {
    /// The value is unset but shouldn't be overwritten
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
