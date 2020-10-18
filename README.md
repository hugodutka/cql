# CQL micro-driver for Rust

This is a very limited async Rust driver for CQL databases. It enables you to do 2 things only:

- establish a connection; and
- send a query.

It won't help you with getting the results of your query or even checking if it executed
successfully.

You may use this crate in your project by adding this dependency to your `Cargo.toml`:

```
cql = { git = "https://github.com/hugodutka/cql" }
```

## Examples:

```rust
use cql::{Connection, Query};

#[tokio::main]
async fn main() {
    let mut conn = Connection::new("localhost:9042").await.unwrap();
    let query = Query::new("INSERT INTO ks.t(a,b,c) VALUES (1,2,'abc')");
    conn.query(query).await.unwrap();
}
```

## Testing

Tests expect a CQL compatible database to be available at `localhost:9042`. You will find a
`docker-compose.yaml` file in the `test_util` folder that can provision one for you. Just run:

```bash
cd tests
docker-compose up
```

You may then run the tests on your host machine with:

```bash
cargo test
```
