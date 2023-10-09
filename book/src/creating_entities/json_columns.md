# JSON columns
If you want something similar to postgres's [`json`](https://www.postgresql.org/docs/current/datatype-json.html) type, you can use the `#[json_data]` attribute macro on a struct.

```rust
#use scyllax::prelude::*;
#
#
/// Represents data from a person
#[json_data]
pub struct PersonData {
    #[serde(rename = "stripeId")]
    pub stripe_id: Option<String>,
}

#[entity]
pub struct PersonEntity {
	#[entity(pk)]
    pub id: uuid::Uuid,
    pub email: String,
	pub data: Option<PersonData>
	#[entity(rename = "createdAt")]
    pub created_at: i64,
}
```

`json_data` uses serde `Deserialize` and `Serialize` under the hood, so you're welcome to use any Serde macro attributes.
