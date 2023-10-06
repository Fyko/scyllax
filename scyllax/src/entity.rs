use scylla::{frame::value::ValueList, FromRow};

/// The traits of the entity
pub trait EntityExt<T: ValueList + FromRow> {
    /// Returns the keys of the entity as a vector of strings, keeping the order of the keys.
    fn keys() -> Vec<String>;

    /// Returns the primary keys
    fn pks() -> Vec<String>;
}
