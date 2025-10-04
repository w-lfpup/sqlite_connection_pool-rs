# Sqlite Connection Pool - rs

A sqlite connection pool using the rusqlite library.

## How to use

```rs
use sqlite_connection_pool::{ConnectionPool};

// filepath, connection count limit
let pool = ConnectionPool::from("./filepath/to/sqlite.db", 8);
let conn = pool.get_connection()?;
let _ = pool.set_connection()?;
```

For a more thread safe experience:
```rs
use sqlite_connection_pool::{from_thread_safe, get_connection, set_connection};

let thread_safe_pool = from_thread_safe("./filepath/to/sqlite.db", 8);
let conn = get_connection(&thread_safe_pool)?;
let _ = set_connection(&thread_safe_pool, conn)?;
```

# License

`sqlite_connection_pool-rs` is released under the BSD 3-Clause License