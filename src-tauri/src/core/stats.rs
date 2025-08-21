use crate::constants::TABLE;
use rusqlite::{Connection, params};

/// Update the daily usage statistics for a specific application (data derived from app usage logs).
pub fn update_app_daily_usage(conn: &Connection, app_name: &str) {
    // 1. Obtain today's date (yyyy-mm-dd)
    let today = chrono::Utc::now().date_naive().to_string();

    // 2. Check if there's an entry for today
    let sql = format!(
        "SELECT total_duration FROM {} WHERE app_name = ? AND date = ?",
        TABLE::DAILY_APP_USAGE
    );
    let mut stmt = conn.prepare(&sql).expect("Error preparing statement");
    let total_duration_result: rusqlite::Result<i64> =
        stmt.query_row(params![app_name, today], |row| row.get(0));
    let total_duration = get_app_usage_duration(&conn, app_name);

    // 2.1 Entry found, update it
    if let Ok(total_duration) = total_duration_result {
        conn.execute(
            "UPDATE app_usage_summary SET total_duration = ? WHERE app_name = ? AND date = ?",
            params![total_duration, app_name, today],
        )
        .expect("Error updating app usage summary");
    }
    // 2.2 Entry not found, create it
    else {
        conn.execute(
            "INSERT INTO app_usage_summary (app_name, date, total_duration) VALUES (?, ?, ?)",
            params![app_name, today, total_duration],
        )
        .expect("Error inserting app usage summary");
    }
}

/// TO-DO
fn get_app_usage_duration(conn: &Connection, app_name: &str) -> i64 {
    let sql = format!(
        "SELECT SUM(duration) FROM {} WHERE app_name = ?",
        TABLE::APP_USAGE_LOGS
    );
    let mut stmt = conn.prepare(&sql).expect("Error preparing statement");
    1
}