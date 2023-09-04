pub mod person {
    pub mod model {
        use scylla::ValueList;
        use scyllax::prelude::*;
        pub struct PersonEntity {
            #[pk]
            pub id: uuid::Uuid,
            pub email: String,
            pub age: Option<i32>,
            #[rename("createdAt")]
            pub created_at: i64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PersonEntity {
            #[inline]
            fn clone(&self) -> PersonEntity {
                PersonEntity {
                    id: ::core::clone::Clone::clone(&self.id),
                    email: ::core::clone::Clone::clone(&self.email),
                    age: ::core::clone::Clone::clone(&self.age),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PersonEntity {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "PersonEntity",
                    "id",
                    &self.id,
                    "email",
                    &self.email,
                    "age",
                    &self.age,
                    "created_at",
                    &&self.created_at,
                )
            }
        }
        impl scylla::_macro_internal::FromRow for PersonEntity {
            fn from_row(
                row: scylla::_macro_internal::Row,
            ) -> ::std::result::Result<Self, scylla::_macro_internal::FromRowError> {
                use scylla::_macro_internal::{
                    CqlValue, FromCqlVal, FromRow, FromRowError,
                };
                use ::std::result::Result::{Ok, Err};
                use ::std::iter::{Iterator, IntoIterator};
                if 4usize != row.columns.len() {
                    return Err(FromRowError::WrongRowSize {
                        expected: 4usize,
                        actual: row.columns.len(),
                    });
                }
                let mut vals_iter = row.columns.into_iter().enumerate();
                Ok(PersonEntity {
                    id: {
                        let (col_ix, col_value) = vals_iter.next().unwrap();
                        <uuid::Uuid as FromCqlVal<
                            ::std::option::Option<CqlValue>,
                        >>::from_cql(col_value)
                            .map_err(|e| FromRowError::BadCqlVal {
                                err: e,
                                column: col_ix,
                            })?
                    },
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
                    age: {
                        let (col_ix, col_value) = vals_iter.next().unwrap();
                        <Option<
                            i32,
                        > as FromCqlVal<
                            ::std::option::Option<CqlValue>,
                        >>::from_cql(col_value)
                            .map_err(|e| FromRowError::BadCqlVal {
                                err: e,
                                column: col_ix,
                            })?
                    },
                    created_at: {
                        let (col_ix, col_value) = vals_iter.next().unwrap();
                        <i64 as FromCqlVal<
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
        impl ::core::marker::StructuralPartialEq for PersonEntity {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PersonEntity {
            #[inline]
            fn eq(&self, other: &PersonEntity) -> bool {
                self.id == other.id && self.email == other.email && self.age == other.age
                    && self.created_at == other.created_at
            }
        }
        impl scylla::_macro_internal::ValueList for PersonEntity {
            fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
                let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                    4usize,
                );
                result.add_value(&self.id)?;
                result.add_value(&self.email)?;
                result.add_value(&self.age)?;
                result.add_value(&self.created_at)?;
                ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
            }
        }
        impl scyllax::EntityExt<PersonEntity> for PersonEntity {
            fn keys() -> Vec<String> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "id".to_string(),
                        "email".to_string(),
                        "age".to_string(),
                        "\"createdAt\"".to_string(),
                    ]),
                )
            }
            fn pks() -> Vec<String> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["id".to_string()]),
                )
            }
        }
        pub struct UpsertPerson {
            pub id: uuid::Uuid,
            pub email: scyllax::prelude::MaybeUnset<String>,
            pub age: scyllax::prelude::MaybeUnset<Option<i32>>,
            pub created_at: scyllax::prelude::MaybeUnset<i64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UpsertPerson {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "UpsertPerson",
                    "id",
                    &self.id,
                    "email",
                    &self.email,
                    "age",
                    &self.age,
                    "created_at",
                    &&self.created_at,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UpsertPerson {
            #[inline]
            fn clone(&self) -> UpsertPerson {
                UpsertPerson {
                    id: ::core::clone::Clone::clone(&self.id),
                    email: ::core::clone::Clone::clone(&self.email),
                    age: ::core::clone::Clone::clone(&self.age),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        impl scyllax::UpsertQuery<PersonEntity> for UpsertPerson {
            fn query(
                &self,
            ) -> Result<
                (String, scyllax::prelude::SerializedValues),
                scyllax::BuildUpsertQueryError,
            > {
                let mut fragments = String::from("update \"person\" set ");
                let mut variables = scylla::frame::value::SerializedValues::new();
                if let scyllax::prelude::MaybeUnset::Set(email) = &self.email {
                    fragments.push_str("\"email\" = ? ");
                    match variables.add_value(email) {
                        Ok(_) => {}
                        Err(
                            scylla::frame::value::SerializeValuesError::TooManyValues,
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                                field: "email".to_string(),
                            });
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::MixingNamedAndNotNamedValues,
                        ) => {
                            return Err(
                                scyllax::BuildUpsertQueryError::MixingNamedAndNotNamedValues,
                            );
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::ValueTooBig(_),
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                                field: "email".to_string(),
                            });
                        }
                        Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                            return Err(scyllax::BuildUpsertQueryError::ParseError {
                                field: "email".to_string(),
                            });
                        }
                    }
                }
                if let scyllax::prelude::MaybeUnset::Set(age) = &self.age {
                    fragments.push_str("\"age\" = ? ");
                    match variables.add_value(age) {
                        Ok(_) => {}
                        Err(
                            scylla::frame::value::SerializeValuesError::TooManyValues,
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                                field: "age".to_string(),
                            });
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::MixingNamedAndNotNamedValues,
                        ) => {
                            return Err(
                                scyllax::BuildUpsertQueryError::MixingNamedAndNotNamedValues,
                            );
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::ValueTooBig(_),
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                                field: "age".to_string(),
                            });
                        }
                        Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                            return Err(scyllax::BuildUpsertQueryError::ParseError {
                                field: "age".to_string(),
                            });
                        }
                    }
                }
                if let scyllax::prelude::MaybeUnset::Set(created_at) = &self.created_at {
                    fragments.push_str("\"\\\"createdAt\\\"\" = ? ");
                    match variables.add_value(created_at) {
                        Ok(_) => {}
                        Err(
                            scylla::frame::value::SerializeValuesError::TooManyValues,
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                                field: "created_at".to_string(),
                            });
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::MixingNamedAndNotNamedValues,
                        ) => {
                            return Err(
                                scyllax::BuildUpsertQueryError::MixingNamedAndNotNamedValues,
                            );
                        }
                        Err(
                            scylla::frame::value::SerializeValuesError::ValueTooBig(_),
                        ) => {
                            return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                                field: "created_at".to_string(),
                            });
                        }
                        Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                            return Err(scyllax::BuildUpsertQueryError::ParseError {
                                field: "created_at".to_string(),
                            });
                        }
                    }
                }
                fragments.push_str("where \"id\" = ?, ");
                match variables.add_value(&self.id) {
                    Ok(_) => {}
                    Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
                        return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                            field: "id".to_string(),
                        });
                    }
                    Err(
                        scylla::frame::value::SerializeValuesError::MixingNamedAndNotNamedValues,
                    ) => {
                        return Err(
                            scyllax::BuildUpsertQueryError::MixingNamedAndNotNamedValues,
                        );
                    }
                    Err(scylla::frame::value::SerializeValuesError::ValueTooBig(_)) => {
                        return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                            field: "id".to_string(),
                        });
                    }
                    Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                        return Err(scyllax::BuildUpsertQueryError::ParseError {
                            field: "id".to_string(),
                        });
                    }
                }
                fragments.pop();
                fragments.pop();
                fragments.push_str(";");
                Ok((fragments, variables))
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
            fn execute<'life0, 'async_trait>(
                self,
                db: &'life0 scyllax::Executor,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<scyllax::QueryResult, scyllax::ScyllaxError>,
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
                            Result<scyllax::QueryResult, scyllax::ScyllaxError>,
                        > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: Result<scyllax::QueryResult, scyllax::ScyllaxError> = {
                        let (query, values) = Self::query(&__self)?;
                        db.session.execute(query, values).await.map_err(|e| e.into())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub mod queries {
        use scyllax::prelude::*;
        use uuid::Uuid;
        pub async fn load(db: &mut Executor) -> anyhow::Result<()> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "load",
                            "scyllax_example::entities::person::queries",
                            tracing::Level::INFO,
                            Some("example/src/entities/person/queries.rs"),
                            Some(4u32),
                            Some("scyllax_example::entities::person::queries"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_instrument_future = async move {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::unreachable,
                    clippy::let_with_type_underscore
                )]
                if false {
                    let __tracing_attr_fake_return: anyhow::Result<()> = {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!(
                                    "this is just for type inference, and is unreachable code",
                                ),
                            ),
                        );
                    };
                    return __tracing_attr_fake_return;
                }
                {
                    let _ = GetPersonById::prepare(db).await;
                    Ok(())
                }
            };
            if !__tracing_attr_span.is_disabled() {
                tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
            } else {
                __tracing_instrument_future.await
            }
        }
        impl scyllax::SelectQuery<
            super::model::PersonEntity,
            Option<super::model::PersonEntity>,
        > for GetPersonById {
            fn query() -> String {
                "select * from person where id = ? limit 1"
                    .replace("*", &super::model::PersonEntity::keys().join(", "))
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
            fn prepare<'life0, 'async_trait>(
                db: &'life0 Executor,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<
                            scylla::prepared_statement::PreparedStatement,
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
                                scylla::prepared_statement::PreparedStatement,
                                scylla::transport::errors::QueryError,
                            >,
                        > {
                        return __ret;
                    }
                    let __ret: Result<
                        scylla::prepared_statement::PreparedStatement,
                        scylla::transport::errors::QueryError,
                    > = {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:11",
                                        "scyllax_example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(11u32),
                                        Some("scyllax_example::entities::person::queries"),
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
                                                    Some(
                                                        &format_args!("preparing query {0}", "GetPersonById")
                                                            as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        db.session
                            .add_prepared_statement(
                                &scylla::query::Query::new(Self::query()),
                            )
                            .await
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
            fn execute<'life0, 'async_trait>(
                self,
                db: &'life0 scyllax::Executor,
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
                        db.session.execute(query, __self).await
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
                            Result<
                                Option<super::model::PersonEntity>,
                                scyllax::ScyllaxError,
                            >,
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
                                                        "event example/src/entities/person/queries.rs:11",
                                                        "scyllax_example::entities::person::queries",
                                                        ::tracing::Level::ERROR,
                                                        Some("example/src/entities/person/queries.rs"),
                                                        Some(11u32),
                                                        Some("scyllax_example::entities::person::queries"),
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
        pub struct GetPersonById {
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
        impl scyllax::SelectQuery<
            super::model::PersonEntity,
            Option<super::model::PersonEntity>,
        > for GetPersonByEmail {
            fn query() -> String {
                "select * from person_by_email where email = ? limit 1"
                    .replace("*", &super::model::PersonEntity::keys().join(", "))
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
            fn prepare<'life0, 'async_trait>(
                db: &'life0 Executor,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<
                            scylla::prepared_statement::PreparedStatement,
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
                                scylla::prepared_statement::PreparedStatement,
                                scylla::transport::errors::QueryError,
                            >,
                        > {
                        return __ret;
                    }
                    let __ret: Result<
                        scylla::prepared_statement::PreparedStatement,
                        scylla::transport::errors::QueryError,
                    > = {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:19",
                                        "scyllax_example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(19u32),
                                        Some("scyllax_example::entities::person::queries"),
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
                                                    Some(
                                                        &format_args!("preparing query {0}", "GetPersonByEmail")
                                                            as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        db.session
                            .add_prepared_statement(
                                &scylla::query::Query::new(Self::query()),
                            )
                            .await
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
            fn execute<'life0, 'async_trait>(
                self,
                db: &'life0 scyllax::Executor,
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
                        db.session.execute(query, __self).await
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
                            Result<
                                Option<super::model::PersonEntity>,
                                scyllax::ScyllaxError,
                            >,
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
                                                        "scyllax_example::entities::person::queries",
                                                        ::tracing::Level::ERROR,
                                                        Some("example/src/entities/person/queries.rs"),
                                                        Some(19u32),
                                                        Some("scyllax_example::entities::person::queries"),
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
        pub struct GetPersonByEmail {
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
    }
}
