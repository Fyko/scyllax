#[derive(thiserror::Error, Debug)]
pub enum ScyllaxError {
    #[error("Scylla Query error: {0}")]
    Scylla(#[from] scylla::transport::errors::QueryError),
    #[error("Scylla single row typed error: {0}")]
    SingleRowTyped(#[from] scylla::transport::query_result::SingleRowTypedError),
    #[error("No rows found")]
    NoRowsFound,

    #[error("Failed to build query: {0}")]
    BuildUpsertQueryError(#[from] BuildUpsertQueryError),
}

#[derive(thiserror::Error, Debug)]
pub enum BuildUpsertQueryError {
    #[error("Too many values when adding {field}")]
    TooManyValues { field: String },
    #[error("Can't mix named and unnamed values")]
    MixingNamedAndNotNamedValues,
    #[error("Value for {field} is too big")]
    ValueTooBig { field: String },
    #[error("Failed to serialize value for {field}")]
    ParseError { field: String },
}
