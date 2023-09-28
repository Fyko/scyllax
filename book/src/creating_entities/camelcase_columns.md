# camelCase Columns
If you have some column names you can't change to work with the Rust naming convention, you can use the `rename` attribute on an entity column to rename it in queries.

```rust
#use scyllax::prelude::*;
#
#[entity]
pub struct PersonEntity {
	#[pk]
    pub id: uuid::Uuid,
    pub email: String,
	#[rename = "createdAt"]
    pub created_at: i64,
}
```
