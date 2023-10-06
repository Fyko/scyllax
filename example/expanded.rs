/// All select queries
pub mod queries {
    use scyllax::prelude::*;
    use uuid::Uuid;
    ///A collection of prepared statements.
    #[allow(non_snake_case)]
    pub struct PersonQueries {
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonById`.
        pub GetPersonById: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPeopleByIds`.
        pub GetPeopleByIds: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonByEmail`.
        pub GetPersonByEmail: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `DeletePersonById`.
        pub DeletePersonById: scylla_reexports::PreparedStatement,
    }
    ///A collection of prepared statements.
    impl scyllax::prelude::QueryCollection for PersonQueries {
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
                    Output = Result<Self, scyllax::prelude::ScyllaxError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Self, scyllax::prelude::ScyllaxError>,
                > {
                    return __ret;
                }
                let __ret: Result<Self, scyllax::prelude::ScyllaxError> = {
                    Ok(Self {
                        GetPersonById: session.prepare(GetPersonById::query()).await?,
                        GetPeopleByIds: session.prepare(GetPeopleByIds::query()).await?,
                        GetPersonByEmail: session
                            .prepare(GetPersonByEmail::query())
                            .await?,
                        DeletePersonById: session
                            .prepare(DeletePersonById::query())
                            .await?,
                    })
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPersonById> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.GetPersonById
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPeopleByIds> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.GetPeopleByIds
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPersonByEmail> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.GetPersonByEmail
        }
    }
    impl scyllax::prelude::GetPreparedStatement<DeletePersonById> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.DeletePersonById
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
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl scyllax::prelude::Query for GetPersonById {
        fn query() -> String {
            "select * from person where id = :id limit 1".to_string()
        }
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();
            values.add_named_value("id", &self.id)?;
            Ok(values)
        }
    }
    impl scyllax::prelude::ReadQuery for GetPersonById {
        type Output = Option<super::model::PersonEntity>;
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
                    Output = Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<Self::Output, scyllax::prelude::ScyllaxError> = {
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
                                                    "event example/src/entities/person/queries.rs:12",
                                                    "example::entities::person::queries",
                                                    ::tracing::Level::ERROR,
                                                    Some("example/src/entities/person/queries.rs"),
                                                    Some(12u32),
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
                                    Err(scyllax::prelude::ScyllaxError::SingleRowTyped(err))
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
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.ids, state);
            ::core::hash::Hash::hash(&self.limit, state)
        }
    }
    impl scyllax::prelude::Query for GetPeopleByIds {
        fn query() -> String {
            "select * from person where id in ? limit ?".to_string()
        }
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();
            values.add_named_value("ids", &self.ids)?;
            values.add_named_value("limit", &self.limit)?;
            Ok(values)
        }
    }
    impl scyllax::prelude::ReadQuery for GetPeopleByIds {
        type Output = Vec<super::model::PersonEntity>;
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
                    Output = Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<Self::Output, scyllax::prelude::ScyllaxError> = {
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
                                            "event example/src/entities/person/queries.rs:22",
                                            "example::entities::person::queries",
                                            ::tracing::Level::ERROR,
                                            Some("example/src/entities/person/queries.rs"),
                                            Some(22u32),
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
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.email, state)
        }
    }
    impl scyllax::prelude::Query for GetPersonByEmail {
        fn query() -> String {
            "select * from person_by_email where email = ? limit 1".to_string()
        }
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();
            values.add_named_value("email", &self.email)?;
            Ok(values)
        }
    }
    impl scyllax::prelude::ReadQuery for GetPersonByEmail {
        type Output = Option<super::model::PersonEntity>;
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
                    Output = Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Self::Output, scyllax::prelude::ScyllaxError>,
                > {
                    return __ret;
                }
                let res = res;
                let __ret: Result<Self::Output, scyllax::prelude::ScyllaxError> = {
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
                                                    "event example/src/entities/person/queries.rs:34",
                                                    "example::entities::person::queries",
                                                    ::tracing::Level::ERROR,
                                                    Some("example/src/entities/person/queries.rs"),
                                                    Some(34u32),
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
                                    Err(scyllax::prelude::ScyllaxError::SingleRowTyped(err))
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
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.id, state)
        }
    }
    impl scyllax::prelude::Query for DeletePersonById {
        fn query() -> String {
            "delete from person where id = :id".to_string()
        }
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();
            values.add_named_value("id", &self.id)?;
            Ok(values)
        }
    }
}
