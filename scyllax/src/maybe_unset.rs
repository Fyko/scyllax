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
