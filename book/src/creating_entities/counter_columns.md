# Counter columns
If you have a table that uses scylla's [`counter`](https://opensource.docs.scylladb.com/stable/cql/types.html#counters) type, you can use the `#[entity(counter)]` attribute macro on an entity column along with using the [`scylla::frame::value::Counter`](https://docs.rs/scylla/latest/scylla/frame/value/struct.Counter.html) type.

```rust
#use scyllax::prelude::*;
#
#[entity]
pub struct PersonLoginEntity {
    #[entity(pk)]
    pub id: uuid::Uuid,
    #[entity(pk)]
    pub person_id: uuid::Uuid,
    #[entity(counter)]
    pub count: scylla::frame::value::Counter,
}
```

Similarly to `#[entity(pk)]`, the `#[entity(counter)]` attribute also tells the upsert macro how to use the column in the query.
