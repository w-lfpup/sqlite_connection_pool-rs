# Sqlite Connection Pool - rs

A sqlite connection pool that is also a facade for the [rusqlite](https://crates.io/crates/rusqlite) library.

## How to use

```rs
use sqlite_connection_pool::{ConnectionPool, Params};

// filepath, connection count limit
let pool = ConnectionPool::from_params(Params {
    db_filepath: PathBuf::from("./filepath/to/sqlite.db")m
    connection_limit: 4,
});

let conn = pool.get_connection()?;
let _ = pool.set_connection(conn)?;
```

For a more thread safe experience use:
```rs
use sqlite_connection_pool::Params;
use sqlite_connection_pool::arcd::{from_params_, get_connection, set_connection};

// Arc<Mutex<ConnectionPool>>
let pool = from_params(Params {
    db_filepath: PathBuf::from("./filepath/to/sqlite.db")m
    connection_limit: 4,
});

let conn = get_connection(&pool)?;
let _ = set_connection(&pool, conn)?;
```

The `get_connection` and `set_connection` are a utility to lock and free mutexes before using a connection.

# License

`sqlite_connection_pool-rs` is released under the BSD 3-Clause License