/// All select queries
pub mod queries {
    use super::model::{UpsertPerson, UpsertPersonWithTTL};
    use scyllax::prelude::*;
    use uuid::Uuid;
    ///A collection of prepared statements.
    #[allow(non_snake_case)]
    pub struct PersonQueries {
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonById`.
        pub get_person_by_id: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPeopleByIds`.
        pub get_people_by_ids: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `GetPersonByEmail`.
        pub get_person_by_email: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `DeletePersonById`.
        pub delete_person_by_id: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `UpsertPerson`.
        pub upsert_person: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The prepared statement for `UpsertPersonWithTTL`.
        pub upsert_person_with_ttl: scylla_reexports::PreparedStatement,
        #[allow(non_snake_case)]
        ///The task for `GetPersonById`.
        pub get_person_by_id_task: Option<
            tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<GetPersonById>>,
        >,
        #[allow(non_snake_case)]
        ///The task for `GetPeopleByIds`.
        pub get_people_by_ids_task: Option<
            tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<GetPeopleByIds>>,
        >,
        #[allow(non_snake_case)]
        ///The task for `GetPersonByEmail`.
        pub get_person_by_email_task: Option<
            tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<GetPersonByEmail>>,
        >,
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    impl ::core::fmt::Debug for PersonQueries {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "get_person_by_id",
                "get_people_by_ids",
                "get_person_by_email",
                "delete_person_by_id",
                "upsert_person",
                "upsert_person_with_ttl",
                "get_person_by_id_task",
                "get_people_by_ids_task",
                "get_person_by_email_task",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.get_person_by_id,
                &self.get_people_by_ids,
                &self.get_person_by_email,
                &self.delete_person_by_id,
                &self.upsert_person,
                &self.upsert_person_with_ttl,
                &self.get_person_by_id_task,
                &self.get_people_by_ids_task,
                &&self.get_person_by_email_task,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "PersonQueries",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_snake_case)]
    impl ::core::clone::Clone for PersonQueries {
        #[inline]
        fn clone(&self) -> PersonQueries {
            PersonQueries {
                get_person_by_id: ::core::clone::Clone::clone(&self.get_person_by_id),
                get_people_by_ids: ::core::clone::Clone::clone(&self.get_people_by_ids),
                get_person_by_email: ::core::clone::Clone::clone(
                    &self.get_person_by_email,
                ),
                delete_person_by_id: ::core::clone::Clone::clone(
                    &self.delete_person_by_id,
                ),
                upsert_person: ::core::clone::Clone::clone(&self.upsert_person),
                upsert_person_with_ttl: ::core::clone::Clone::clone(
                    &self.upsert_person_with_ttl,
                ),
                get_person_by_id_task: ::core::clone::Clone::clone(
                    &self.get_person_by_id_task,
                ),
                get_people_by_ids_task: ::core::clone::Clone::clone(
                    &self.get_people_by_ids_task,
                ),
                get_person_by_email_task: ::core::clone::Clone::clone(
                    &self.get_person_by_email_task,
                ),
            }
        }
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
                        get_person_by_id: scyllax::prelude::prepare_query(
                                &session,
                                GetPersonById::query(),
                            )
                            .await?,
                        get_people_by_ids: scyllax::prelude::prepare_query(
                                &session,
                                GetPeopleByIds::query(),
                            )
                            .await?,
                        get_person_by_email: scyllax::prelude::prepare_query(
                                &session,
                                GetPersonByEmail::query(),
                            )
                            .await?,
                        delete_person_by_id: scyllax::prelude::prepare_query(
                                &session,
                                DeletePersonById::query(),
                            )
                            .await?,
                        upsert_person: scyllax::prelude::prepare_query(
                                &session,
                                UpsertPerson::query(),
                            )
                            .await?,
                        upsert_person_with_ttl: scyllax::prelude::prepare_query(
                                &session,
                                UpsertPersonWithTTL::query(),
                            )
                            .await?,
                        get_person_by_id_task: None,
                        get_people_by_ids_task: None,
                        get_person_by_email_task: None,
                    })
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn register_tasks(
            mut self,
            executor: std::sync::Arc<scyllax::prelude::Executor<Self>>,
        ) -> Self {
            self
                .get_person_by_id_task = {
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                let ex = executor.clone();
                tokio::spawn(async move {
                    ex.read_task::<GetPersonById>(rx).await;
                });
                Some(tx)
            };
            self
                .get_people_by_ids_task = {
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                let ex = executor.clone();
                tokio::spawn(async move {
                    ex.read_task::<GetPeopleByIds>(rx).await;
                });
                Some(tx)
            };
            self
                .get_person_by_email_task = {
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                let ex = executor.clone();
                tokio::spawn(async move {
                    ex.read_task::<GetPersonByEmail>(rx).await;
                });
                Some(tx)
            };
            self
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPersonById> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.get_person_by_id
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPeopleByIds> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.get_people_by_ids
        }
    }
    impl scyllax::prelude::GetPreparedStatement<GetPersonByEmail> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.get_person_by_email
        }
    }
    impl scyllax::prelude::GetPreparedStatement<DeletePersonById> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.delete_person_by_id
        }
    }
    impl scyllax::prelude::GetPreparedStatement<UpsertPerson> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.upsert_person
        }
    }
    impl scyllax::prelude::GetPreparedStatement<UpsertPersonWithTTL> for PersonQueries {
        ///Get a prepared statement.
        fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
            &self.upsert_person_with_ttl
        }
    }
    impl scyllax::prelude::GetCoalescingSender<GetPersonById> for PersonQueries {
        ///Get a task.
        fn get(
            &self,
        ) -> &tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<GetPersonById>> {
            &self.get_person_by_id_task.as_ref().unwrap()
        }
    }
    impl scyllax::prelude::GetCoalescingSender<GetPeopleByIds> for PersonQueries {
        ///Get a task.
        fn get(
            &self,
        ) -> &tokio::sync::mpsc::Sender<
            scyllax::executor::ShardMessage<GetPeopleByIds>,
        > {
            &self.get_people_by_ids_task.as_ref().unwrap()
        }
    }
    impl scyllax::prelude::GetCoalescingSender<GetPersonByEmail> for PersonQueries {
        ///Get a task.
        fn get(
            &self,
        ) -> &tokio::sync::mpsc::Sender<
            scyllax::executor::ShardMessage<GetPersonByEmail>,
        > {
            &self.get_person_by_email_task.as_ref().unwrap()
        }
    }
    /// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
    #[read_query(
        query = "select * from person where id = :id limit 1",
        return_type = "super::model::PersonEntity"
    )]
    pub struct GetPersonById {
        /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
        #[read_query(coalesce_shard_key)]
        pub id: Uuid,
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
    impl scylla::_macro_internal::ValueList for GetPersonById {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                1usize,
            );
            result.add_value(&self.id)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scyllax::prelude::Query for GetPersonById {
        fn query() -> String {
            "select * from person where id = :id limit 1"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
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
                                    Err(scyllax::error::ScyllaxError::SingleRowTyped(err))
                                }
                            }
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn shard_key(&self) -> String {
            [self.id.to_string()].join(":")
        }
    }
    /// Get many [`super::model::PersonEntity`] by many [`uuid::Uuid`]
    #[read_query(
        query = "select * from person where id in :ids limit :rowlimit",
        return_type = "Vec<super::model::PersonEntity>"
    )]
    pub struct GetPeopleByIds {
        /// The [`uuid::Uuid`]s of the [`super::model::PersonEntity`]s to get
        pub ids: Vec<Uuid>,
        /// The maximum number of [`super::model::PersonEntity`]s to get
        pub rowlimit: i32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GetPeopleByIds {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "GetPeopleByIds",
                "ids",
                &self.ids,
                "rowlimit",
                &&self.rowlimit,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GetPeopleByIds {
        #[inline]
        fn clone(&self) -> GetPeopleByIds {
            GetPeopleByIds {
                ids: ::core::clone::Clone::clone(&self.ids),
                rowlimit: ::core::clone::Clone::clone(&self.rowlimit),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GetPeopleByIds {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GetPeopleByIds {
        #[inline]
        fn eq(&self, other: &GetPeopleByIds) -> bool {
            self.ids == other.ids && self.rowlimit == other.rowlimit
        }
    }
    impl scylla::_macro_internal::ValueList for GetPeopleByIds {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                2usize,
            );
            result.add_value(&self.ids)?;
            result.add_value(&self.rowlimit)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scyllax::prelude::Query for GetPeopleByIds {
        fn query() -> String {
            "select * from person where id in :ids limit :rowlimit"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
        }
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();
            values.add_named_value("ids", &self.ids)?;
            values.add_named_value("rowlimit", &self.rowlimit)?;
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
                                            "event example/src/entities/person/queries.rs:24",
                                            "example::entities::person::queries",
                                            ::tracing::Level::ERROR,
                                            Some("example/src/entities/person/queries.rs"),
                                            Some(24u32),
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
        fn shard_key(&self) -> String {
            String::new()
        }
    }
    /// Get a [`super::model::PersonEntity`] by its email address
    #[read_query(
        query = "select * from person_by_email where email = :email limit 1",
        return_type = "super::model::PersonEntity"
    )]
    pub struct GetPersonByEmail {
        /// The email address of the [`super::model::PersonEntity`] to get
        pub email: String,
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
    impl scylla::_macro_internal::ValueList for GetPersonByEmail {
        fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
            let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                1usize,
            );
            result.add_value(&self.email)?;
            ::std::result::Result::Ok(::std::borrow::Cow::Owned(result))
        }
    }
    impl scyllax::prelude::Query for GetPersonByEmail {
        fn query() -> String {
            "select * from person_by_email where email = :email limit 1"
                .replace("*", &super::model::PersonEntity::keys().join(", "))
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
                                                    "event example/src/entities/person/queries.rs:37",
                                                    "example::entities::person::queries",
                                                    ::tracing::Level::ERROR,
                                                    Some("example/src/entities/person/queries.rs"),
                                                    Some(37u32),
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
                                    Err(scyllax::error::ScyllaxError::SingleRowTyped(err))
                                }
                            }
                        }
                    }
                };
                #[allow(unreachable_code)] __ret
            })
        }
        fn shard_key(&self) -> String {
            String::new()
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
    impl scyllax::prelude::WriteQuery for DeletePersonById {}
}
