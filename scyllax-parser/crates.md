# scyllax-parser (sɪl-æks)
A parser for CQL queries.

[![discord](https://img.shields.io/discord/1080316613968011335?color=5865F2&logo=discord&logoColor=white)](https://discord.gg/FahQSBMMGg)
[![codecov](https://codecov.io/gh/trufflehq/scyllax/graph/badge.svg?token=OGH77YR0TA)](https://codecov.io/gh/trufflehq/scyllax)
[![CI](https://github.com/trufflehq/scyllax/actions/workflows/ci.yml/badge.svg)](https://github.com/trufflehq/scyllax/actions/workflows/ci.yml)

## Usage
```rust
use scyllax_parser::{Column, Query, SelectQuery};

let query = Query::try_from("select id, name from person");

assert_eq!(
    query,
    Ok(Query::Select(SelectQuery {
        table: "person".to_string(),
        columns: vec![
            Column::Identifier("id".to_string()),
            Column::Identifier("name".to_string()),
        ],
        condition: vec![],
        limit: None,
    }))
);
```
