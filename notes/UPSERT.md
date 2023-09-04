For an upsert query, there will be a little bit of database magic. For the sake of getting my ideas onto paper as quick as possible, the entity in this will be `Person`.

We have the following structs:
- `PersonEntity`, the entity itself
- `UpsertPerson`, the generated struct that has every field but pks wrapped in `MaybeUnset`.

An upsert query will be provided like this:
```protobuf
service PersonHypha {
	rpc UpsertPerson(UpsertPersonRequest) returns (PersonEntity);
}

message Person {
	// v1 uuid
	string id = 1;
	// the name of the user
	string first_name = 2;
	/// the email of the user
	string email = 3;
	/// optional, the age of the user
	optional int age = 4;
}

// The request passed to the upsert query
message UpsertPersonRequest {
	// the base
	PersonEntity head = 1;
	// the update diff
	UpsertPersonDiff diff = 2;
}

// The content of the diff
// this can't include the `id` because its a primary key that can't be altered
message UpsertPersonDiff {
	// the first name of the user
	optional string first_name = 1;
	// the email of the user
	optional string email = 2;
	// the age of the user
	optional int age = 3;
}
```

Now, the gRPC server recieves the request:
```rs
fn upsert_person(UpsertPersonRequest { head, diff }: UpsertPersonRequest) -> todo!() {

}
```

Now that the server has the request, it needs to:
- pull out the PKs for the `where` clause of the query
- loop over the diff and pull out the fields that are set

For example, given the following request:

```rust
UpsertPersonRequest {
	head: PersonEntity {
		id: "123e4567-e89b-12d3-a456-426614174000",
		first_name: "John",
		email: "john@google.com",
		age: 20,
	},
	diff: UpsertPersonDiff {
		age: 21
	}
}
```

Should generate the query:
```cql
update person set age = ? where id = ?;
-- variables: (21, 123e4567-e89b-12d3-a456-426614174000)
```

And, since this is upsert, we use the same query for an insert, where the head is empty:
```rust
UpsertPersonRequest {
	head: PersonEntity {},
	diff: UpsertPersonDiff {
		id: "123e4567-e89b-12d3-a456-426614174000",
		first_name: "John",
		email: "john@google.com",
		age: 20,
	}
}
```

Should generate the query:
```cql
update person set first_name = ?, email = ?, age = ? where id = ?;
-- variables: ("John", "john@google.com", 20, 123e4567-e89b-12d3-a456-426614174000)
```

So, there will be one struct to be used for the query, named `UpsertPerson`.
We have to put the primary keys at the end of the struct, so that they can be pulled out for the `where` clause last.
```rust
struct UpsertPerson {
	first_name: MaybeUnset<String>,
	email: MaybeUnset<String>,
	age: MaybeUnset<Optional<i32>>,
	id: String,
}
```
`MaybeUnset` just means the value may not be set. Instead of updating the value, we will just ignore it.

`UpsertPerson` implements the `UpsertQuery` trait, which looks like:
```rust
/// The trait that's implemented on update/insert queryes
#[async_trait]
pub trait UpsertQuery<
    T: EntityExt<T> + ValueList + FromRow,
>
{
    /// Returns the query as a string
    fn query(&self) -> String;

    /// Prepares the query
    async fn prepare(db: &Executor) -> Result<PreparedStatement, QueryError>;

    /// Executes the query
    async fn execute(self, db: &Executor) -> Result<QueryResult, QueryError>;

    /// Parses the response from the database
    async fn parse_response(res: QueryResult) -> Result<(), ScyllaxError>;
}
```

The `query` function is most crutial, as it considers every `MaybeUnset` field and generates the query.
```rust
impl UpsertQuery<PersonEntity> for UpsertPerson {
	fn query(&self) -> (String) {
		let mut query = String::from("update person set ");
		let mut variables = scylla::frame::value::SerializedValues::new();

		if let MaybeUnset::Set(first_name) = &self.first_name {
			query.push_str("first_name = ?, ");
			variables
		}

		if let MaybeUnset::Set(email) = &self.email {
			query.push_str("email = ?, ");
			variables.push(email);
		}

		if let MaybeUnset::Set(age) = &self.age {
			// age is also optional, so we have to unwrap it
			if let Some(age) = age {
				query.push_str("age = ?, ");
				variables.push(age);
			}
		}

		query.pop();
		query.pop();
		query.push_str(" where id = ?;");
		variables.push(&self.id);

		query
	}
}
```
