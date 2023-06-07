# postgres-json-derive

Postgres derive based on this [issue](https://github.com/sfackler/rust-postgres/issues/622)

## Usage

```rust
use postgres_json_derive::{ToSqlJson, FromSqlJson};

#[derive(ToSqlJson, FromSqlJson)]
struct Horizon {
    zero: String,
    dawn: i32,
}
```