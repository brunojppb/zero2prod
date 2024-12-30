# Zero2Prod

An email newsletter API built in Rust to follow along the [Zero to Production in Rust book](https://www.zero2prod.com/index.html)

## Development

Using cargo-watch to auto-reload/compile the server

```shell
# Install cargo-watch first with: cargo install cargo-watch˜
cargo watch -x check -x run
```

## Generating sqlx offline files

sqlx macros scan the codebase for SQL queries and run them
against the database to make them type-safe.
To generate the offline files so you don't need a database connection,
run the following cargo command:

```shell
cargo sqlx prepare -- --all-targets --all-features
```

Now you can run a `cargo check` with `SQLX_OFFLINE=true` and a database connection isn't required.
