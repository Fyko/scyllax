//! Macros for matching rows from a [`scylla::QueryResult`]
#[macro_export]
/// Take a QueryResult and return a `Result<Option<T>>`
/// Example:
/// ```rust,ignore
/// match_row!(res, OrgEntity)
/// ```
macro_rules! match_row {
    ($res:ident, $type:ty) => {
        match $res.single_row_typed::<$type>() {
            Ok(data) => Ok(Some(data)),
            Err(err) => {
                use scylla::transport::query_result::SingleRowTypedError;
                match err {
                    // tried to parse into type, but there are no rows
                    SingleRowTypedError::BadNumberOfRows(_) => Ok(None),
                    _ => {
                        tracing::error!("err: {:?}", err);
                        Err(scyllax::prelude::ScyllaxError::SingleRowTyped(err))
                    }
                }
            }
        }
    };
}

#[macro_export]
/// Take a QueryResult and return a `Result<Vec<T>>`
/// Example:
/// ```rust,ignore
/// match_rows!(res, OrgEntity)
/// ```
macro_rules! match_rows {
    ($res:ident, $type:ty) => {
        match $res.rows_typed::<$type>() {
            Ok(xs) => Ok(xs.filter_map(|x| x.ok()).collect::<Vec<$type>>()),
            Err(e) => {
                tracing::error!("err: {:?}", e);

                Ok(vec![])
            }
        }
    };
}
