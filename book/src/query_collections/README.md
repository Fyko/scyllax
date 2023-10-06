# Query Collections
Once you've made all your queries, you'll have to make a Query Collection. This is a struct that contains all your prepared queries. Scyllax provides the `create_query_collection` macro to make this easy.

```rust
use scyllax::prelude::*;
use super::model::UpsertPerson;

create_query_collection!(
    PersonQueries,
    [
        GetPersonById,
        GetPeopleByIds,
        GetPersonByEmail,
        DeletePersonById,
        UpsertPerson,
    ]
);
```
Then, you use the Query Collection when you instantiate your Executor.

```rust
let executor = Executor::<PersonQueries>::new(session).await?;
```

Finally, you can run your queries.

```rust
let person = executor
	.execute_read(&GetPersonById {
		id: Uuid::new_v4(),
	})
	.await?;
```
