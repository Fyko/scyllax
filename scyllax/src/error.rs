#[derive(thiserror::Error, Debug)]
pub enum ScyllaxError {
    #[error("Scylla Query error: {0}")]
    Scylla(#[from] scylla::transport::errors::QueryError),
    #[error("Scylla single row typed error: {0}")]
    SingleRowTyped(#[from] scylla::transport::query_result::SingleRowTypedError),
    #[error("No rows found")]
    NoRowsFound,
}
