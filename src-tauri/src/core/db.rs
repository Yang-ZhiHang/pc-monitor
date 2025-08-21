use crate::constants::{DB_NAME, TABLE};
use rusqlite::{Connection, Error};

pub fn init_db() -> Result<(), Error> {
    let conn = Connection::open(DB_NAME)?;

    let tables = [
        (
            TABLE::APP_USAGE_LOGS,
            "timestamp TEXT NOT NULL, window_title TEXT NOT NULL",
        ),
        (
            TABLE::DAILY_APP_USAGE,
            "date DATE NOT NULL, window_title TEXT NOT NULL, total_duration INTEGER NOT NULL",
        ),
        (
            TABLE::DAILY_USAGE_STATS,
            "date DATE NOT NULL, window_title TEXT NOT NULL, total_duration INTEGER NOT NULL",
        ),
    ];

    for (table_name, columns) in tables {
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY,
                    {}
                )",
                table_name, columns
            ),
            [],
        )?;
    }

    Ok(())
}

pub fn insert(table_name: &str, params: &[&dyn rusqlite::ToSql]) -> Result<(), Error> {
    let conn = Connection::open(DB_NAME)?;

    let query = match table_name {
        TABLE::APP_USAGE_LOGS => {
            "INSERT INTO app_usage_logs (timestamp, window_title) VALUES (?, ?)"
        }
        TABLE::DAILY_APP_USAGE => {
            "INSERT INTO daily_app_usage (date, window_title, total_duration) VALUES (?, ?, ?)"
        }
        TABLE::DAILY_USAGE_STATS => {
            "INSERT INTO daily_usage_stats (date, window_title, total_duration) VALUES (?, ?, ?)"
        }
        _ => return Err(Error::InvalidParameterCount(0, 0)),
    };

    conn.execute(query, params)?;
    Ok(())
}
