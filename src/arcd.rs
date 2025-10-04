use crate::{ConnectionPool, Params};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn from_params(params: Params) -> Arc<Mutex<ConnectionPool>> {
    Arc::new(Mutex::new(ConnectionPool::from_params(params)))
}

// function to get connection and free mutex
pub fn get_connection(pool: &Arc<Mutex<ConnectionPool>>) -> Result<Connection, String> {
    let mut connections = match pool.lock() {
        Ok(connections) => connections,
        Err(e) => return Err(e.to_string()),
    };

    connections.get_connection()
}

// utility to set connection and free mutex
pub fn set_connection(pool: &Arc<Mutex<ConnectionPool>>, conn: Connection) -> Result<(), String> {
    let mut connections = match pool.lock() {
        Ok(connections) => connections,
        Err(e) => return Err(e.to_string()),
    };

    connections.set_connection(conn)
}
