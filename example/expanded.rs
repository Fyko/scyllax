pub mod person {
    //! The Person entity
    /// The model itself
    pub mod model {
        use scyllax::prelude::*;
        /// Represents data from a person
        pub struct PersonData {
            /// The stripe id of the person
            #[serde(rename = "stripeId")]
            pub stripe_id: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PersonData {
            #[inline]
            fn clone(&self) -> PersonData {
                PersonData {
                    stripe_id: ::core::clone::Clone::clone(&self.stripe_id),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PersonData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "PersonData",
                    "stripe_id",
                    &&self.stripe_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PersonData {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PersonData {
            #[inline]
            fn eq(&self, other: &PersonData) -> bool {
                self.stripe_id == other.stripe_id
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PersonData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "PersonData",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "stripeId",
                        &self.stripe_id,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PersonData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "stripeId" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"stripeId" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<PersonData>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PersonData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct PersonData",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PersonData with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(PersonData { stripe_id: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "stripeId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("stripeId")?
                                }
                            };
                            _serde::__private::Ok(PersonData { stripe_id: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["stripeId"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PersonData",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<PersonData>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl scylla::frame::value::Value for PersonData {
            fn serialize(
                &self,
                buf: &mut Vec<u8>,
            ) -> Result<(), scylla::frame::value::ValueTooBig> {
                let data = serde_json::to_vec(self).unwrap();
                <Vec<u8> as scylla::frame::value::Value>::serialize(&data, buf)
            }
        }
        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue>
        for PersonData {
            fn from_cql(
                cql_val: scylla::frame::response::result::CqlValue,
            ) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                let data = <Vec<
                    u8,
                > as scylla::cql_to_rust::FromCqlVal<
                    scylla::frame::response::result::CqlValue,
                >>::from_cql(cql_val)?;
                serde_json::from_slice(&data)
                    .ok()
                    .ok_or(scylla::cql_to_rust::FromCqlValError::BadCqlType)
            }
        }
        /// Represents a person in the database
        pub struct PersonEntity {
            /// The id of the person
            #[pk]
            pub id: uuid::Uuid,
            /// The email address of the person
            pub email: String,
            /// The age of the person
            pub age: Option<i32>,
            /// Other data from the person
            pub data: Option<PersonData>,
            /// The date the person was created
            #[rename("createdAt")]
            pub created_at: i64,
        }
        ///Upserts a PersonEntity into the `person` table
        pub struct UpsertPerson {
            ///The id of the PersonEntity
            pub id: uuid::Uuid,
            ///The email of the PersonEntity
            pub email: scyllax::prelude::MaybeUnset<String>,
            ///The age of the PersonEntity
            pub age: scyllax::prelude::MaybeUnset<Option<i32>>,
            ///The data of the PersonEntity
            pub data: scyllax::prelude::MaybeUnset<Option<PersonData>>,
            ///The created_at of the PersonEntity
            pub created_at: scyllax::prelude::MaybeUnset<i64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UpsertPerson {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "UpsertPerson",
                    "id",
                    &self.id,
                    "email",
                    &self.email,
                    "age",
                    &self.age,
                    "data",
                    &self.data,
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
                    data: ::core::clone::Clone::clone(&self.data),
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
                let query = "update person set email = :email, age = :age, data = :data, \"createdAt\" = :created_at where id = :id;"
                    .to_string();
                let mut variables = scylla::frame::value::SerializedValues::new();
                match variables.add_named_value("email", &self.email) {
                    Ok(_) => {}
                    Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
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
                    Err(scylla::frame::value::SerializeValuesError::ValueTooBig(_)) => {
                        return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                            field: "email".to_string(),
                        });
                    }
                    Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                        return Err(scyllax::BuildUpsertQueryError::ParseError {
                            field: "email".to_string(),
                        });
                    }
                };
                match variables.add_named_value("age", &self.age) {
                    Ok(_) => {}
                    Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
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
                    Err(scylla::frame::value::SerializeValuesError::ValueTooBig(_)) => {
                        return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                            field: "age".to_string(),
                        });
                    }
                    Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                        return Err(scyllax::BuildUpsertQueryError::ParseError {
                            field: "age".to_string(),
                        });
                    }
                };
                match variables.add_named_value("data", &self.data) {
                    Ok(_) => {}
                    Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
                        return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                            field: "data".to_string(),
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
                            field: "data".to_string(),
                        });
                    }
                    Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                        return Err(scyllax::BuildUpsertQueryError::ParseError {
                            field: "data".to_string(),
                        });
                    }
                };
                match variables.add_named_value("created_at", &self.created_at) {
                    Ok(_) => {}
                    Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
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
                    Err(scylla::frame::value::SerializeValuesError::ValueTooBig(_)) => {
                        return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                            field: "created_at".to_string(),
                        });
                    }
                    Err(scylla::frame::value::SerializeValuesError::ParseError) => {
                        return Err(scyllax::BuildUpsertQueryError::ParseError {
                            field: "created_at".to_string(),
                        });
                    }
                };
                match variables.add_named_value("id", &self.id) {
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
                };
                Ok((query, variables))
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
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/model.rs:13",
                                        "example::entities::person::model",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/model.rs"),
                                        Some(13u32),
                                        Some("example::entities::person::model"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "query", "values"],
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
                                                    Some(&format_args!("executing upsert") as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&debug(&query) as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&values.len() as &dyn Value),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        db.session.execute(query, values).await.map_err(|e| e.into())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PersonEntity {
            #[inline]
            fn clone(&self) -> PersonEntity {
                PersonEntity {
                    id: ::core::clone::Clone::clone(&self.id),
                    email: ::core::clone::Clone::clone(&self.email),
                    age: ::core::clone::Clone::clone(&self.age),
                    data: ::core::clone::Clone::clone(&self.data),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PersonEntity {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "PersonEntity",
                    "id",
                    &self.id,
                    "email",
                    &self.email,
                    "age",
                    &self.age,
                    "data",
                    &self.data,
                    "created_at",
                    &&self.created_at,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PersonEntity {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PersonEntity {
            #[inline]
            fn eq(&self, other: &PersonEntity) -> bool {
                self.id == other.id && self.email == other.email && self.age == other.age
                    && self.data == other.data && self.created_at == other.created_at
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
                if 5usize != row.columns.len() {
                    return Err(FromRowError::WrongRowSize {
                        expected: 5usize,
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
                    data: {
                        let (col_ix, col_value) = vals_iter.next().unwrap();
                        <Option<
                            PersonData,
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
        impl scylla::_macro_internal::ValueList for PersonEntity {
            fn serialized(&self) -> scylla::_macro_internal::SerializedResult {
                let mut result = scylla::_macro_internal::SerializedValues::with_capacity(
                    5usize,
                );
                result.add_value(&self.id)?;
                result.add_value(&self.email)?;
                result.add_value(&self.age)?;
                result.add_value(&self.data)?;
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
                        "data".to_string(),
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
    }
    /// All select queries
    pub mod queries {
        use scyllax::{delete_query, prelude::*};
        use uuid::Uuid;
        /// Load all queries for this entity
        pub async fn load(db: &mut Executor) -> anyhow::Result<()> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "load",
                            "example::entities::person::queries",
                            tracing::Level::INFO,
                            Some("example/src/entities/person/queries.rs"),
                            Some(5u32),
                            Some("example::entities::person::queries"),
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
                    let _ = GetPeopleByIds::prepare(db).await;
                    let _ = GetPersonByEmail::prepare(db).await;
                    let _ = DeletePersonById::prepare(db).await;
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
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.id, state)
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
                                        "event example/src/entities/person/queries.rs:16",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(16u32),
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
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:16",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(16u32),
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
                                                    Some(&format_args!("executing select") as &dyn Value),
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
                                                        "event example/src/entities/person/queries.rs:16",
                                                        "example::entities::person::queries",
                                                        ::tracing::Level::ERROR,
                                                        Some("example/src/entities/person/queries.rs"),
                                                        Some(16u32),
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
        impl scyllax::SelectQuery<
            super::model::PersonEntity,
            Vec<super::model::PersonEntity>,
        > for GetPeopleByIds {
            fn query() -> String {
                "select * from person where id in ? limit ?"
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
                                        "event example/src/entities/person/queries.rs:26",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(26u32),
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
                                                    Some(
                                                        &format_args!("preparing query {0}", "GetPeopleByIds")
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
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:26",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(26u32),
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
                                                    Some(&format_args!("executing select") as &dyn Value),
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
                            Result<
                                Vec<super::model::PersonEntity>,
                                scyllax::ScyllaxError,
                            >,
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
                                                "event example/src/entities/person/queries.rs:26",
                                                "example::entities::person::queries",
                                                ::tracing::Level::ERROR,
                                                Some("example/src/entities/person/queries.rs"),
                                                Some(26u32),
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
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.email, state)
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
                                        "event example/src/entities/person/queries.rs:38",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(38u32),
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
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:38",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(38u32),
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
                                                    Some(&format_args!("executing select") as &dyn Value),
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
                                                        "event example/src/entities/person/queries.rs:38",
                                                        "example::entities::person::queries",
                                                        ::tracing::Level::ERROR,
                                                        Some("example/src/entities/person/queries.rs"),
                                                        Some(38u32),
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
        impl scyllax::DeleteQuery<super::model::PersonEntity> for DeletePersonById {
            fn query() -> String {
                "delete from person where id = ?".to_string()
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
                        let query = Self::query();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:48",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(48u32),
                                        Some("example::entities::person::queries"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message", "target", "query"],
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
                                                    Some(&format_args!("preparing query") as &dyn Value),
                                                ),
                                                (
                                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                    Some(&"DeletePersonById" as &dyn Value),
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
                        db.session
                            .add_prepared_statement(&scylla::query::Query::new(query))
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
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event example/src/entities/person/queries.rs:48",
                                        "example::entities::person::queries",
                                        ::tracing::Level::DEBUG,
                                        Some("example/src/entities/person/queries.rs"),
                                        Some(48u32),
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
}
