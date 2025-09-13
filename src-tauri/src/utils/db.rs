use super::file::get_exe_path;
use crate::{
    constants::db::{DB_NAME, TABLE},
    singleton_with_logging,
};
use parking_lot::Mutex;
use rusqlite::Connection;

pub struct DbManager {
    pub handle: Mutex<Connection>,
}

impl DbManager {
    pub fn new() -> Self {
        Self {
            handle: Mutex::new(init_db().expect("Failed to initialize database")),
        }
    }

    pub fn get(&self) -> &Mutex<Connection> {
        &self.handle
    }
}

singleton_with_logging!(DbManager, DB_CONN);

/// Initialize the database and create necessary tables.
pub fn init_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let exe_path = get_exe_path()?;
    let db_path = exe_path
        .parent()
        .ok_or("Failed to get exe parent dir")?
        .join(DB_NAME);
    let conn = Connection::open(&db_path)?;

    let tables = [
        (
            TABLE::APP_USAGE_LOGS,
            "time DATETIME NOT NULL, app_name TEXT NOT NULL",
        ),
        (
            TABLE::DAILY_APP_USAGE,
            "date DATETIME NOT NULL, app_name TEXT NOT NULL, total_usage INTEGER NOT NULL, UNIQUE (date, app_name)",
        ),
        (
            TABLE::DAILY_USAGE_STATS,
            "date DATETIME NOT NULL, total_usage INTEGER NOT NULL, UNIQUE (date)",
        ),
    ];

    for (table_name, columns) in tables {
        let _ = conn
            .execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY,
                    {}
                )",
                    table_name, columns
                ),
                [],
            )
            .map_err(|e| <rusqlite::Error as Into<Box<dyn std::error::Error>>>::into(e));
    }

    Ok(conn)
}

/// Insert a new record into the specified table.
///
/// ## Example
///
/// ```ignore
/// use super::*;
///
/// let params = params![
///     "2023-01-01 12:00:00",
///     "Visual Studio Code"
/// ];
///
/// let result = insert(TABLE::APP_USAGE_LOGS, &params);
/// ```
pub fn insert(
    conn: &Connection,
    table_name: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<(), rusqlite::Error> {
    let query = match table_name {
        TABLE::APP_USAGE_LOGS => "INSERT INTO app_usage_logs (time, app_name) VALUES (?, ?)",
        TABLE::DAILY_APP_USAGE => {
            "INSERT INTO daily_app_usage (date, app_name, total_usage) VALUES (?, ?, ?)"
        }
        TABLE::DAILY_USAGE_STATS => {
            "INSERT INTO daily_usage_stats (date, total_usage) VALUES (?, ?)"
        }
        _ => {
            return Err(rusqlite::Error::InvalidParameterName(format!(
                "Invalid table name: {}",
                table_name
            )));
        }
    };

    conn.execute(query, params)?;
    Ok(())
}
