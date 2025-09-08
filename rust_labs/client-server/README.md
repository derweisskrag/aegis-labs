# Client-Server

## SQLx CLI:

https://docs.rs/crate/sqlx-cli/latest

## Database creation:

Run the command:

```
cargo sqlx database create
```

> It will read the env file and create the database.

After you can run the command

```
cargo sqlx migrate run
```

We can also run the command: `cargo sqlx prepare` to use `query!`, `query_as!` macros!

### With SQLite, Postgres and also MySQL: 

```
cargo install sqlx-cli --no-default-features --features sqlite,postgres,mysql
```