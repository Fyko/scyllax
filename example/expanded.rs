/// All select queries
pub mod queries {
    use scyllax::PreparedStatement;
    #[allow(non_snake_case)]
    use scyllax::{delete_query, prelude::*};
    use uuid::Uuid;
    ///A collection of prepared statements.
    #[allow(non_snake_case)]
    pub struct PersonEntityQueries {
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonById`.
        pub GetPersonById: scylla::statement::prepared_statement::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPeopleByIds`.
        pub GetPeopleByIds: scylla::statement::prepared_statement::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonByEmail`.
        pub GetPersonByEmail: scylla::statement::prepared_statement::PreparedStatement,
    }
    ///A collection of prepared statements.
    impl scyllax::Queries for PersonEntityQueries {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn new<'life0, 'async_trait>(
            session: &'life0 scylla::Session,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Self, scyllax::ScyllaxError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<Self, scyllax::ScyllaxError>,
                    > {
                    return __ret;
                }
                let __ret: Result<Self, scyllax::ScyllaxError> = {
                    Ok(Self {
                        GetPersonById: session.prepare(GetPersonById::query()).await?,
                        GetPeopleByIds: session.prepare(GetPeopleByIds::query()).await?,
                        GetPersonByEmail: session
                            .prepare(GetPersonByEmail::query())
                            .await?,
                    })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        ///Get a prepared statement.
        fn get<T>(&self) -> &scylla::statement::prepared_statement::PreparedStatement
        where
            Self: scyllax::GetPreparedStatement<T>,
        {
            <Self as scyllax::GetPreparedStatement<T>>::get_prepared_statement(self)
        }
    }
    impl scyllax::GetPreparedStatement<GetPersonById> for PersonEntityQueries {
        ///Get a prepared statement.
        fn get_prepared_statement(
            &self,
        ) -> &scylla::statement::prepared_statement::PreparedStatement {
            &self.GetPersonById
        }
    }
    impl scyllax::GetPreparedStatement<GetPeopleByIds> for PersonEntityQueries {
        ///Get a prepared statement.
        fn get_prepared_statement(
            &self,
        ) -> &scylla::statement::prepared_statement::PreparedStatement {
            &self.GetPeopleByIds
        }
    }
    impl scyllax::GetPreparedStatement<GetPersonByEmail> for PersonEntityQueries {
        ///Get a prepared statement.
        fn get_prepared_statement(
            &self,
        ) -> &scylla::statement::prepared_statement::PreparedStatement {
            &self.GetPersonByEmail
        }
    }
    /// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
    pub struct GetPersonById {
        /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
        pub id: Uuid,
    }
    impl scylla::_macro_internal::ValueList for GetPersonById {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                1usize,
            );
            result.add_value(&self.id)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scylla::_macro_internal::FromRow for GetPersonById {
        fn from_row(
            row: scylla::_macro_internal::Row,
        ) -> ::std::result::Result<Self, scylla::_macro_internal::FromRowError> {
            use scylla::_macro_internal::{CqlValue, FromCqlVal, FromRow, FromRowError};
            use ::std::result::Result::{Ok, Err};
            use ::std::iter::{Iterator, IntoIterator};
            if 1usize != row.columns.len() {
                return Err(FromRowError::WrongRowSize {
                    expected: 1usize,
                    actual: row.columns.len(),
                });
            }
            let mut vals_iter = row.columns.into_iter().enumerate();
            Ok(GetPersonById {
                id: {
                    let (col_ix, col_value) = vals_iter.next().unwrap();
                    <Uuid as FromCqlVal<
                        ::std::option::Option<CqlValue>,
                    >>::from_cql(col_value)
                        .map_err(|e| FromRowError::BadCqlVal {
                            err: e,
                            column: col_ix,
                        })?
                },
            })
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GetPersonById {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "GetPersonById",
                "id",
                &&self.id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GetPersonById {
        #[inline]
        fn clone(&self) -> GetPersonById {
            GetPersonById {
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GetPersonById {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GetPersonById {
        #[inline]
        fn eq(&self, other: &GetPersonById) -> bool {
            self.id == other.id
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for GetPersonById {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl scyllax::GenericQuery<super::model::PersonEntity> for GetPersonById {
        fn query() -> String {
            "select * from person where id = ? limit 1"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
        }
    }
    impl scyllax::SelectQuery<
        super::model::PersonEntity,
        Option<super::model::PersonEntity>,
        super::model::PersonEntity,
    > for GetPersonById {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn execute<'life0, 'async_trait>(
            self,
            db: &'life0 scyllax::Executor<super::model::PersonEntity>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        scylla::QueryResult,
                        scylla::transport::errors::QueryError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<
                            scylla::QueryResult,
                            scylla::transport::errors::QueryError,
                        >,
                    > {
                    return __ret;
                }
                let __self = self;
                let __ret: Result<
                    scylla::QueryResult,
                    scylla::transport::errors::QueryError,
                > = {
                    let statement = db.queries.get::<GetPersonById>();
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event example/src/entities/person/queries.rs:19",
                                    "example::entities::person::queries",
                                    ::tracing::Level::DEBUG,
                                    Some("example/src/entities/person/queries.rs"),
                                    Some(19u32),
                                    Some("example::entities::person::queries"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("executing select") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    db.session.execute(statement, __self).await
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn parse_response<'async_trait>(
            res: scylla::QueryResult,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        Option<super::model::PersonEntity>,
                        scyllax::ScyllaxError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<Option<super::model::PersonEntity>, scyllax::ScyllaxError>,
                    > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<
                    Option<super::model::PersonEntity>,
                    scyllax::ScyllaxError,
                > = {
                    match res.single_row_typed::<super::model::PersonEntity>() {
                        Ok(data) => Ok(Some(data)),
                        Err(err) => {
                            use scylla::transport::query_result::SingleRowTypedError;
                            match err {
                                SingleRowTypedError::BadNumberOfRows(_) => Ok(None),
                                _ => {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event example/src/entities/person/queries.rs:19",
                                                    "example::entities::person::queries",
                                                    ::tracing::Level::ERROR,
                                                    Some("example/src/entities/person/queries.rs"),
                                                    Some(19u32),
                                                    Some("example::entities::person::queries"),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = CALLSITE.metadata().fields().iter();
                                                CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                                Some(&format_args!("err: {0:?}", err) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Err(scyllax::ScyllaxError::SingleRowTyped(err))
                                }
                            }
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    /// Get many [`super::model::PersonEntity`] by many [`uuid::Uuid`]
    pub struct GetPeopleByIds {
        /// The [`uuid::Uuid`]s of the [`super::model::PersonEntity`]s to get
        pub ids: Vec<Uuid>,
        /// The maximum number of [`super::model::PersonEntity`]s to get
        pub limit: i32,
    }
    impl scylla::_macro_internal::ValueList for GetPeopleByIds {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                2usize,
            );
            result.add_value(&self.ids)?;
            result.add_value(&self.limit)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scylla::_macro_internal::FromRow for GetPeopleByIds {
        fn from_row(
            row: scylla::_macro_internal::Row,
        ) -> ::std::result::Result<Self, scylla::_macro_internal::FromRowError> {
            use scylla::_macro_internal::{CqlValue, FromCqlVal, FromRow, FromRowError};
            use ::std::result::Result::{Ok, Err};
            use ::std::iter::{Iterator, IntoIterator};
            if 2usize != row.columns.len() {
                return Err(FromRowError::WrongRowSize {
                    expected: 2usize,
                    actual: row.columns.len(),
                });
            }
            let mut vals_iter = row.columns.into_iter().enumerate();
            Ok(GetPeopleByIds {
                ids: {
                    let (col_ix, col_value) = vals_iter.next().unwrap();
                    <Vec<
                        Uuid,
                    > as FromCqlVal<
                        ::std::option::Option<CqlValue>,
                    >>::from_cql(col_value)
                        .map_err(|e| FromRowError::BadCqlVal {
                            err: e,
                            column: col_ix,
                        })?
                },
                limit: {
                    let (col_ix, col_value) = vals_iter.next().unwrap();
                    <i32 as FromCqlVal<
                        ::std::option::Option<CqlValue>,
                    >>::from_cql(col_value)
                        .map_err(|e| FromRowError::BadCqlVal {
                            err: e,
                            column: col_ix,
                        })?
                },
            })
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GetPeopleByIds {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "GetPeopleByIds",
                "ids",
                &self.ids,
                "limit",
                &&self.limit,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GetPeopleByIds {
        #[inline]
        fn clone(&self) -> GetPeopleByIds {
            GetPeopleByIds {
                ids: ::core::clone::Clone::clone(&self.ids),
                limit: ::core::clone::Clone::clone(&self.limit),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GetPeopleByIds {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GetPeopleByIds {
        #[inline]
        fn eq(&self, other: &GetPeopleByIds) -> bool {
            self.ids == other.ids && self.limit == other.limit
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for GetPeopleByIds {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.ids, state);
            ::core::hash::Hash::hash(&self.limit, state)
        }
    }
    impl scyllax::GenericQuery<super::model::PersonEntity> for GetPeopleByIds {
        fn query() -> String {
            "select * from person where id in ? limit ?"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
        }
    }
    impl scyllax::SelectQuery<
        super::model::PersonEntity,
        Vec<super::model::PersonEntity>,
        super::model::PersonEntity,
    > for GetPeopleByIds {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn execute<'life0, 'async_trait>(
            self,
            db: &'life0 scyllax::Executor<super::model::PersonEntity>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        scylla::QueryResult,
                        scylla::transport::errors::QueryError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<
                            scylla::QueryResult,
                            scylla::transport::errors::QueryError,
                        >,
                    > {
                    return __ret;
                }
                let __self = self;
                let __ret: Result<
                    scylla::QueryResult,
                    scylla::transport::errors::QueryError,
                > = {
                    let statement = db.queries.get::<GetPeopleByIds>();
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event example/src/entities/person/queries.rs:82",
                                    "example::entities::person::queries",
                                    ::tracing::Level::DEBUG,
                                    Some("example/src/entities/person/queries.rs"),
                                    Some(82u32),
                                    Some("example::entities::person::queries"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("executing select") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    db.session.execute(statement, __self).await
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn parse_response<'async_trait>(
            res: scylla::QueryResult,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        Vec<super::model::PersonEntity>,
                        scyllax::ScyllaxError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<Vec<super::model::PersonEntity>, scyllax::ScyllaxError>,
                    > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<
                    Vec<super::model::PersonEntity>,
                    scyllax::ScyllaxError,
                > = {
                    match res.rows_typed::<super::model::PersonEntity>() {
                        Ok(xs) => {
                            Ok(
                                xs
                                    .filter_map(|x| x.ok())
                                    .collect::<Vec<super::model::PersonEntity>>(),
                            )
                        }
                        Err(e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event example/src/entities/person/queries.rs:82",
                                            "example::entities::person::queries",
                                            ::tracing::Level::ERROR,
                                            Some("example/src/entities/person/queries.rs"),
                                            Some(82u32),
                                            Some("example::entities::person::queries"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = CALLSITE.metadata().fields().iter();
                                        CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                        Some(&format_args!("err: {0:?}", e) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Ok(::alloc::vec::Vec::new())
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    /// Get a [`super::model::PersonEntity`] by its email address
    pub struct GetPersonByEmail {
        /// The email address of the [`super::model::PersonEntity`] to get
        pub email: String,
    }
    impl scylla::_macro_internal::ValueList for GetPersonByEmail {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                1usize,
            );
            result.add_value(&self.email)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scylla::_macro_internal::FromRow for GetPersonByEmail {
        fn from_row(
            row: scylla::_macro_internal::Row,
        ) -> ::std::result::Result<Self, scylla::_macro_internal::FromRowError> {
            use scylla::_macro_internal::{CqlValue, FromCqlVal, FromRow, FromRowError};
            use ::std::result::Result::{Ok, Err};
            use ::std::iter::{Iterator, IntoIterator};
            if 1usize != row.columns.len() {
                return Err(FromRowError::WrongRowSize {
                    expected: 1usize,
                    actual: row.columns.len(),
                });
            }
            let mut vals_iter = row.columns.into_iter().enumerate();
            Ok(GetPersonByEmail {
                email: {
                    let (col_ix, col_value) = vals_iter.next().unwrap();
                    <String as FromCqlVal<
                        ::std::option::Option<CqlValue>,
                    >>::from_cql(col_value)
                        .map_err(|e| FromRowError::BadCqlVal {
                            err: e,
                            column: col_ix,
                        })?
                },
            })
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GetPersonByEmail {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "GetPersonByEmail",
                "email",
                &&self.email,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GetPersonByEmail {
        #[inline]
        fn clone(&self) -> GetPersonByEmail {
            GetPersonByEmail {
                email: ::core::clone::Clone::clone(&self.email),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GetPersonByEmail {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GetPersonByEmail {
        #[inline]
        fn eq(&self, other: &GetPersonByEmail) -> bool {
            self.email == other.email
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for GetPersonByEmail {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.email, state)
        }
    }
    impl scyllax::GenericQuery<super::model::PersonEntity> for GetPersonByEmail {
        fn query() -> String {
            "select * from person_by_email where email = ? limit 1"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
        }
    }
    impl scyllax::SelectQuery<
        super::model::PersonEntity,
        Option<super::model::PersonEntity>,
        super::model::PersonEntity,
    > for GetPersonByEmail {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn execute<'life0, 'async_trait>(
            self,
            db: &'life0 scyllax::Executor<super::model::PersonEntity>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        scylla::QueryResult,
                        scylla::transport::errors::QueryError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<
                            scylla::QueryResult,
                            scylla::transport::errors::QueryError,
                        >,
                    > {
                    return __ret;
                }
                let __self = self;
                let __ret: Result<
                    scylla::QueryResult,
                    scylla::transport::errors::QueryError,
                > = {
                    let statement = db.queries.get::<GetPersonByEmail>();
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event example/src/entities/person/queries.rs:94",
                                    "example::entities::person::queries",
                                    ::tracing::Level::DEBUG,
                                    Some("example/src/entities/person/queries.rs"),
                                    Some(94u32),
                                    Some("example::entities::person::queries"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("executing select") as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    db.session.execute(statement, __self).await
                };
                #[allow(unreachable_code)] __ret
            })
        }
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn parse_response<'async_trait>(
            res: scylla::QueryResult,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<
                        Option<super::model::PersonEntity>,
                        scyllax::ScyllaxError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        Result<Option<super::model::PersonEntity>, scyllax::ScyllaxError>,
                    > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<
                    Option<super::model::PersonEntity>,
                    scyllax::ScyllaxError,
                > = {
                    match res.single_row_typed::<super::model::PersonEntity>() {
                        Ok(data) => Ok(Some(data)),
                        Err(err) => {
                            use scylla::transport::query_result::SingleRowTypedError;
                            match err {
                                SingleRowTypedError::BadNumberOfRows(_) => Ok(None),
                                _ => {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event example/src/entities/person/queries.rs:94",
                                                    "example::entities::person::queries",
                                                    ::tracing::Level::ERROR,
                                                    Some("example/src/entities/person/queries.rs"),
                                                    Some(94u32),
                                                    Some("example::entities::person::queries"),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = CALLSITE.metadata().fields().iter();
                                                CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                                Some(&format_args!("err: {0:?}", err) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    Err(scyllax::ScyllaxError::SingleRowTyped(err))
                                }
                            }
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    /// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
    pub struct DeletePersonById {
        /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
        pub id: Uuid,
    }
    impl scylla::_macro_internal::ValueList for DeletePersonById {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                1usize,
            );
            result.add_value(&self.id)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DeletePersonById {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DeletePersonById",
                "id",
                &&self.id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DeletePersonById {
        #[inline]
        fn clone(&self) -> DeletePersonById {
            DeletePersonById {
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DeletePersonById {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DeletePersonById {
        #[inline]
        fn eq(&self, other: &DeletePersonById) -> bool {
            self.id == other.id
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DeletePersonById {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl scyllax::GenericQuery<super::model::PersonEntity> for DeletePersonById {
        fn query() -> String {
            "delete from person where id = ?".to_string()
        }
    }
    impl scyllax::DeleteQuery<super::model::PersonEntity, "# inner_entity_typeQueries">
    for DeletePersonById {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn execute<'life0, 'async_trait>(
            self,
            db: &'life0 scyllax::Executor<"# inner_entity_typeQueries">,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<
                        scylla::QueryResult,
                        scylla::transport::errors::QueryError,
                    >,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret)
                    = ::core::option::Option::None::<
                        anyhow::Result<
                            scylla::QueryResult,
                            scylla::transport::errors::QueryError,
                        >,
                    > {
                    return __ret;
                }
                let __self = self;
                let __ret: anyhow::Result<
                    scylla::QueryResult,
                    scylla::transport::errors::QueryError,
                > = {
                    let query = Self::query();
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event example/src/entities/person/queries.rs:104",
                                    "example::entities::person::queries",
                                    ::tracing::Level::DEBUG,
                                    Some("example/src/entities/person/queries.rs"),
                                    Some(104u32),
                                    Some("example::entities::person::queries"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message", "query"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&format_args!("executing delete") as &dyn Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&query as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    db.session.execute(query, __self).await
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
