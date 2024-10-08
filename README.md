# scyllax (sɪl-æks)
A SQLx and Discord inspired query system for Scylla.

[![discord](https://img.shields.io/discord/1041931589631881257?color=5865F2&logo=discord&logoColor=white)](https://discord.gg/HnyYTnQzJW)
[![codecov](https://codecov.io/gh/fyko/scyllax/graph/badge.svg?token=OGH77YR0TA)](https://codecov.io/gh/fyko/scyllax)
[![CI](https://github.com/fyko/scyllax/actions/workflows/ci.yml/badge.svg)](https://github.com/fyko/scyllax/actions/workflows/ci.yml)

## Example
### 1. Model definition
Before you can write any queries, you have to define a model.
```rust,ignore
#[entity]
pub struct PersonEntity {
    #[entity(primary_key)]
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: i64,
}
```

### 2. Read queries
With the [`read_query`] attribute, it's easy to define select queries.
```rust,ignore
#[read_query(
    query = "select * from person where id = :id limit 1",
    return_type = "PersonEntity"
)]
pub struct GetPersonById {
    pub id: Uuid,
}
```

### 3. Upsert queries
With the [`upsert_query`] attribute, it's easy to define upsert queries.
```rust,ignore
#[entity]
#[upsert_query(table = "person", name = UpsertPerson)]
pub struct PersonEntity {
    #[entity(primary_key)]
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: i64,
}
```

### 4. Query Collections
Scylla required queries be prepared before they can be executed. To prepare (and check) all queries at startup, create a query collection and pass it into an Executor.
```rust,ignore
create_query_collection!(
    PersonQueries,
    [
        GetPersonById,
        GetPersonByEmail
    ],
    [
        DeletePersonById,
        UpsertPerson
    ]
);

let executor = Executor::<PersonQueries>::new(Arc::new(session)).await;

let user = executor.execute_read(GetPersonByEmail {
    email: "user@fyko.net".to_string(),
}).await?;

println!("{user:#?}");
```

## Features
- [x] Read Queries
- [x] Write Queries (https://github.com/fyko/scyllax/pull/1)
- [x] Request Coalescing
- [x] Compile-time Select Query Validation
  - ensure the where constraints exist on the struct
  - ensure the where constraints are the same type as the struct
- [ ] Runtime Query Validation (structure matches schema)

### Todo
- [x] Eject `anyhow`, more refined errors

## Usage
See the [example](example) for more details.

## References
1. https://www.reddit.com/r/rust/comments/11ki2n7/a_look_at_how_discord_uses_rust_for_their_data/jb8dmrx/

```rs
#[read_request(
    query = "select * from foo where id = ? limit 1",
    entity_type = "Foo"
)]
struct GetFooById {
    #[shard_key]
    id: i64
}
```

```rs
handle.execute_read(GetFooById { ... }).await
```

**Messages from Jake**

> the answer though is that unlike the scylla rust wrapper, we don't need the fields to be in the right order for our stuff to work.
> we do 2 clever things:

> 1) `SELECT *` is actually a lie. Never use `SELECT *` in a prepared statement, **ever**. CQL suffers from a protocol level bug that can lead to data corruption on schema change when doing a `SELECT *` due to a schema de-sync condition that is possible between the client & server. So instead, what we do is, we look at the entity type struct, and we transform `SELECT *` into `SELECT col_a, col_b, col_c`. That means if a column present in the schema, but not in the struct we're going to de-serialize to, we don't actually query it. The gist of the bug is that, when a new column is added to a table, the database may start returning data for that column, without the client being aware of that. In the pathological case, this can cause a mis-aligned deserialziation of the data. https://docs.datastax.com/en/developer/java-driver/3.0/manual/statements/prepared/#avoid-preparing-select-queries - although this does look like it's finally fixed in native protocol v5, I'm unsure if scylla is using that yet.

> 2) For binding of the query parameters as well, we essentially parse the SQL statement and figure out all of the bind positions, and then generate code that will bind the fields in the proper order (since on the wire level, they need to be specified in the order that they're defined in the query.) We do this at compile time in a proc macro to generate the code that does the serialization of the query, so we incur no runtime overhead of re-ordering things.

> At startup we prepare everything and also type check the structs in code against what's in the db
Registering everything manually is fine
You can make it fail at compile time
If you try to use an unregistered query
