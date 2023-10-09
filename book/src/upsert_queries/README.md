# Upsert Queries
Now listen up. Upsert queries with scyllax are special. You don't have to hand write anything. In conjunction with `#[pk]`, `#[counter]`, and `#[json]`, the `upsert_query` macro will generate the query for you. Simply apply the `#[upsert_query]` macro to tne entity struct.

```rust
#use scyllax::prelude::*;
#
#[entity]
#[upsert_query(table = "person", name = UpsertPerson)]
pub struct PersonEntity {
	#[entity(pk)]
    pub id: uuid::Uuid,
    pub email: String,
	#[entity(rename = "createdAt")]
    pub created_at: i64,
}
```

The structure will look a little like:
```rust,ignore
pub struct UpsertPerson {
	///The id of the PersonEntity
	pub id: uuid::Uuid,
	///The email of the PersonEntity
	pub email: scyllax::prelude::MaybeUnset<String>,
	///The created_at of the PersonEntity
	pub created_at: scyllax::prelude::MaybeUnset<i64>,
}
```
and the generated query will look a little like:
```cql
update person set email = :email, age = :age, data = :data, \"createdAt\" = :created_at where id = :id;
```

`MaybeUnset` is used to tell scylla if the field is not provided to the query, it should be ignored, and not overwritten. Every [Value](https://docs.rs/scylla/latest/scylla/frame/value/trait.Value.html) can be used with `MaybeUnset`.

Once you've built your query, you can pass it to an Executor.
```rust,ignore
let id = v1_uuid();

let query = UpsertPerson {
	id,
	email: "foo@scyllax.local".to_string().into(),
	created_at: MaybeUnset::Unset,
};

let res = executor.execute_upsert(query).await?;
```
