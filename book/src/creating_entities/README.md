# Creating entities

Creating a new entity is super simple. Simply use the [`scyllax::Entity`](https://docs.rs/scyllax/latest/scyllax/derive.Entity.html) macro on a struct, and it'll implement the necessary symbols and traits.

```rust
#use scyllax::prelude::*;
#
#[derive(Clone, Debug, PartialEq, Entity, FromRow, ValueList)]
pub struct PersonEntity {
	#[pk]
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: i64,
}
```
Since `id` is a partition key, it must be annotated with `#[pk]`.
**Clustering columns must be treated the same way**.

This is so that, when eventually using the `upsert_query` macro, scyllax will use the column in the where clause rather than the set clause.

You're also welcome to use the `#[entity]` macro instead of deriving `Clone`, `Debug`, `PartialEq`, `Entity`, `FromRow`, and `ValueList` manually. That's what'll be used in the rst of this book.

A list of possible column types can be found at [scylla::frame::response::result::CqlValue](https://docs.rs/scylla/latest/scylla/frame/response/result/enum.CqlValue.html).
