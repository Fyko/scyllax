# Select Queries
Writing select queries is incredibly easy with the `select_query` macro.

Simply create a struct with the fields you want to select, and annotate it with the `#[select_query]` macro.

```rust
#use scyllax::prelude::*;
#
#\#[entity]
#pub struct PersonEntity {
#	#[pk]
#    pub id: uuid::Uuid,
#    pub email: String,
#	#[rename = "createdAt"]
#    pub created_at: i64,
#}
#
#[read_query(
    query = "select * from person where id = ? limit 1",
    entity_type = "PersonEntity"
)]
pub struct GetPersonById {
    pub id: Uuid,
}
```

Then, you can pass it to the executor you made in [Introduction](../index.html).
```rust,ignore
let query = GetPersonById {
    id: Uuid::from_str("00000000-0000-0000-0000-000000000000")?,
};

let res = executor
    .execute_select(query)
    .await?
```
