use chrono::{Duration, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

static DB: OnceCell<Arc<RustQLite>> = OnceCell::const_new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    id: i32,
    address: String,
    tx_type: String,
    date: String,
}

pub struct RustQLite {
    conn: Mutex<Connection>,
}

impl RustQLite {
    pub async fn init() -> Arc<Self> {
        DB.get_or_init(|| async {
            let instance = Arc::new(RustQLite::new());
            instance.create_table().await;
            instance
        })
        .await
        .clone()
    }

    fn new() -> Self {
        let conn = Connection::open("database.db")
            .unwrap_or_else(|err| panic!("Error initializing database: {}", err));
        RustQLite {
            conn: Mutex::new(conn),
        }
    }

    async fn create_table(&self) {
        let conn = self.conn.lock().await;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address TEXT NOT NULL,
                tx_type TEXT NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        )
        .unwrap_or_else(|err| panic!("Failed to create table: {}", err));
    }

    pub async fn insert_log(address: &str, tx_type: &str) {
        let db = RustQLite::init().await;
        let now = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let conn = db.conn.lock().await;
        conn.execute(
            "INSERT INTO log (address, tx_type, date) VALUES (?1, ?2, ?3)",
            params![address, tx_type, now],
        )
        .unwrap_or_else(|err| panic!("Error inserting log data: {}", err));
    }

    pub async fn update_log(id: i32, new_address: &str, new_tx_type: &str) {
        let db = RustQLite::init().await;
        let now = Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let conn = db.conn.lock().await;
        let rows_affected = conn
            .execute(
                "UPDATE log SET address = ?1, tx_type = ?2, date = ?3 WHERE id = ?4",
                params![new_address, new_tx_type, now, id],
            )
            .unwrap_or_else(|err| panic!("Error updating log: {}", err));

        if rows_affected == 0 {
            panic!("No log found with ID: {}", id);
        }
    }

    pub async fn delete_log(id: i32) {
        let db = RustQLite::init().await;
        let conn = db.conn.lock().await;
        let rows_affected = conn
            .execute("DELETE FROM log WHERE id = ?1", params![id])
            .unwrap_or_else(|err| panic!("Error deleting log: {}", err));

        if rows_affected == 0 {
            panic!("No log found with ID: {}", id);
        }
    }

    pub async fn get_all_logs() -> Vec<Log> {
        let db = RustQLite::init().await;
        let conn = db.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT id, address, tx_type, date FROM log")
            .unwrap_or_else(|err| panic!("Error preparing statement: {}", err));

        let rows = stmt
            .query_map([], |row| {
                Ok(Log {
                    id: row.get(0)?,
                    address: row.get(1)?,
                    tx_type: row.get(2)?,
                    date: row.get(3)?,
                })
            })
            .unwrap_or_else(|err| panic!("Error querying logs: {}", err));

        rows.collect::<Result<Vec<Log>, rusqlite::Error>>()
            .unwrap_or_else(|err| panic!("Error collecting logs: {}", err))
    }

    pub async fn get_logs_today(address: &str, tx_type: &str) -> Vec<Log> {
        let db = RustQLite::init().await;
        let conn = db.conn.lock().await;

        let now = Utc::now().naive_utc();
        let today_start = now.date().and_hms_opt(0, 0, 0).unwrap();
        let today_end = today_start + Duration::days(1);

        let start_str = today_start.format("%Y-%m-%d %H:%M:%S").to_string();
        let end_str = today_end.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut stmt = conn
            .prepare(
                "SELECT id, address, tx_type, date FROM log 
                WHERE address = ?1 AND tx_type = ?2 
                AND date BETWEEN ?3 AND ?4",
            )
            .unwrap_or_else(|err| panic!("Error preparing statement: {}", err));

        let rows = stmt
            .query_map(params![address, tx_type, start_str, end_str], |row| {
                Ok(Log {
                    id: row.get(0)?,
                    address: row.get(1)?,
                    tx_type: row.get(2)?,
                    date: row.get(3)?,
                })
            })
            .unwrap_or_else(|err| panic!("Error querying logs: {}", err));

        rows.collect::<Result<Vec<Log>, rusqlite::Error>>()
            .unwrap_or_else(|err| panic!("Error collecting logs: {}", err))
    }
}
