pub use rusqlite::Connection;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct ConnectionPool {
    db_path: String,
    connection_limit: usize,
    incoming_connections: Vec<Connection>,
    outgoing_connections: Vec<Connection>,
}

impl ConnectionPool {
    pub fn from(db_path: &str, connection_limit: usize) -> ConnectionPool {
        ConnectionPool {
            db_path: db_path.to_string(),
            incoming_connections: Vec::new(),
            outgoing_connections: Vec::new(),
            connection_limit,
        }
    }

    pub fn get_connection(&mut self) -> Result<Connection, String> {
        if let Some(conn) = self.pop() {
            return Ok(conn);
        }

        match Connection::open(&self.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn set_connection(&mut self, conn: Connection) -> Result<(), String> {
        self.pop();

        let connection_count = self.outgoing_connections.len() + self.incoming_connections.len();
        if connection_count < self.connection_limit {
            self.incoming_connections.push(conn);
        }

        Ok(())
    }

    fn pop(&mut self) -> Option<Connection> {
        if self.outgoing_connections.len() == 0 {
            while let Some(connection) = self.incoming_connections.pop() {
                self.outgoing_connections.push(connection);
            }
        }

        self.outgoing_connections.pop()
    }
}

pub fn from_thread_safe(db_path: &str, connection_limit: usize) -> Arc<Mutex<ConnectionPool>> {
    Arc::new(Mutex::new(ConnectionPool::from(db_path, connection_limit)))
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
