use sqlite_connection_pool::{Connection, from_thread_safe, get_connection, set_connection};
use std::fs;
use std::path::PathBuf;

const test_db_filepath: &str = "sqlite_connection_pool_tests.sqlite";

#[test]
fn test_lifecycle() -> Result<(), String> {
    let db_path = PathBuf::from(test_db_filepath);
    let pool = from_thread_safe(&db_path, 1);

    let mut conn = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Err(e),
    };

    // pass connection as mutable reference to preserve ownership in test function scope
    let message = get_hello_world_sqlite(&mut conn);
    assert_eq!(Ok(1), message);

    if let Err(e) = set_connection(&pool, conn) {
        return Err(e);
    }

    let mut conn2 = match get_connection(&pool) {
        Ok(conn) => conn,
        Err(e) => return Err(e),
    };

    let message = get_hello_world_sqlite(&mut conn2);

    if let Err(e) = set_connection(&pool, conn2) {
        return Err(e);
    }

    if let Err(e) = fs::remove_file(&db_path) {
        return Err(e.to_string());
    };

    Ok(())
}

fn get_hello_world_sqlite(conn: &mut Connection) -> Result<usize, String> {
    let mut stmt = match conn.prepare("SELECT 1") {
        Ok(stmt) => stmt,
        Err(e) => return Err(e.to_string()),
    };

    let mut iter = match stmt.query_map([], |row| {
        let message: usize = row.get(0)?;
        Ok(message)
    }) {
        Ok(iter) => iter,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let entry = match iter.next() {
        Some(entry) => entry,
        _ => {
            return Err("connection did not return an entry".to_string());
        }
    };

    match entry {
        Ok(entry) => Ok(entry),
        Err(e) => Err(e.to_string()),
    }
}
