# Getting Started

Scyllax is a query system (and ORM kinda) for Scylla. It is a work in progress and is not ready for production use.

# Installation
Although scyllax depends on scylla internally, it's recommended to have it installed if you decide to use some values from scylla.
Additionally, while in alpha, there is no `tracing` flag, so you must also install tracing.

```toml
scylla = "0.9"
scyllax = "0.1.0"
tracing = "0.1"
```

Scyllax's prelude includes everything you'll need, so import it at the top of your file:

```rust
use scyllax::prelude::*;
```

# Creating an executor
Your queries will need to be ran by an Executor. Creating one is simple. You can use the [`create_session`](https://docs.rs/scyllax/0.1.8-alpha/scyllax/executor/fn.create_session.html) function provided by scyllax, and pass it to a new Executor.

```rust
#use scyllax::prelude::*;
#
#[tokio::main]
fn main() {
	let known_nodes = std::env::var("SCYLLA_NODES")
		.expect("SCYLLA_NODES must be set");
    let known_nodes = known_nodes.split(',').collect::<Vec<_>>();
    let default_keyspace = std::env::var("SCYLLA_DEFAULT_KEYSPACE").ok();

    let session = create_session(known_nodes, default_keyspace).await?;
    let executor = Executor::<UserQueries>::new(session).await?;
}
```
