use crate::constants::TABLE;
use crate::utils::db::init_db;
use crate::utils::test::jsonify;
use chrono::{Duration, Local, NaiveDate, TimeZone, Utc};
use log::debug;
use rusqlite::{Connection, params};
use std::collections::HashMap;

/// Update the daily usage statistics for a specific application (data derived from app usage logs).
pub fn update_daily_app_usage(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 1. Obtain today's date (yyyy-mm-dd)
    let utc_today = chrono::Utc::now().date_naive().to_string();

    // 2. Update usage duration for each app
    let local_today = Local::now().date_naive();
    let usage_map = get_app_usage_duration(&conn, local_today)?;
    for (key, val) in &usage_map {
        let sql = format!(
            "INSERT INTO {} (date, app_name, total_usage) VALUES (?, ?, ?) ON CONFLICT(date, app_name) DO UPDATE SET total_usage = ?",
            TABLE::DAILY_APP_USAGE
        );
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute(params![utc_today, key, val, val])?;
    }
    Ok(())
}

/// Update the daily usage statistics for all applications (data derived from daily app usage stats).
pub fn update_daily_usage_stats(conn: &Connection) -> Result<(), rusqlite::Error> {
    let today = chrono::Utc::now().date_naive().to_string();

    // Update usage duration for today
    let duration = get_daily_usage_duration(&conn)?;
    let sql = format!(
        "INSERT INTO {} (date, total_usage) VALUES (?, ?) ON CONFLICT(date) DO UPDATE SET total_usage = ?",
        TABLE::DAILY_USAGE_STATS
    );
    let mut stmt = conn.prepare(&sql)?;
    stmt.execute(params![today, duration, duration])?;
    Ok(())
}

/// Get all app usage durations that occurred today
pub fn get_app_usage_duration(conn: &Connection, local_date: NaiveDate) -> Result<HashMap<String, i64>, rusqlite::Error> {
    let (start_of_day, end_of_day) = get_local_day_start_end_in_utc(local_date);

    let sql = format!(
        "SELECT * FROM {} WHERE time BETWEEN '{}' AND '{}'",
        TABLE::APP_USAGE_LOGS,
        start_of_day,
        end_of_day
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;

    // Get the first row as 'pre'
    let mut pre_time: Option<String> = None;
    let mut pre_name: Option<String> = None;
    if let Some(row) = rows.next()? {
        pre_time = Some(row.get::<_, String>(1)?);
        pre_name = Some(row.get::<_, String>(2)?);
    }
    if pre_name.is_none() {
        return Ok(HashMap::new());
    }

    // TO-DO: if PC is sleepping, the duration may not be recorded
    // Traverse from the second row onwards
    let mut result: HashMap<String, i64> = HashMap::new();
    while let Some(row) = rows.next()? {
        let cur_time = row.get::<_, String>(1)?;
        let cur_name = row.get::<_, String>(2)?;
        if let (Ok(pre_dt), Ok(cur_dt)) = (
            chrono::NaiveDateTime::parse_from_str(&pre_time.as_ref().unwrap(), "%Y-%m-%d %H:%M:%S"),
            chrono::NaiveDateTime::parse_from_str(&cur_time, "%Y-%m-%d %H:%M:%S"),
        ) {
            let duration = cur_dt.signed_duration_since(pre_dt).num_seconds();
            result
                .entry(pre_name.as_ref().unwrap().clone())
                .and_modify(|v| *v += duration)
                .or_insert(duration);
        }
        pre_time = Some(cur_time);
        pre_name = Some(cur_name);
    }
    debug!("Total app usage duration: {}", jsonify(&result));
    Ok(result)
}

/// Get all app usage durations that occurred today
fn get_daily_usage_duration(conn: &Connection) -> Result<i64, rusqlite::Error> {
    let local_today = get_local_date_in_utc();

    let sql = format!(
        "SELECT * FROM {} WHERE date = '{}'",
        TABLE::DAILY_APP_USAGE,
        local_today
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;

    // Summation
    let mut total_duration = 0;
    while let Some(row) = rows.next()? {
        total_duration += row.get::<_, i64>(3)?;
    }

    Ok(total_duration)
}

/// Obtain the starting and ending times of the { local_date }
fn get_local_day_start_end_in_utc(local_date: NaiveDate) -> (String, String) {
    let local_start = local_date.and_hms_opt(0, 0, 0).unwrap();
    let local_end = local_date.and_hms_opt(23, 59, 59).unwrap();
    // 转为 UTC
    let utc_start = Local
        .from_local_datetime(&local_start)
        .unwrap()
        .with_timezone(&Utc);
    let utc_end = Local
        .from_local_datetime(&local_end)
        .unwrap()
        .with_timezone(&Utc);
    let start_of_day = utc_start.format("%Y-%m-%d %H:%M:%S").to_string();
    let end_of_day = utc_end.format("%Y-%m-%d %H:%M:%S").to_string();
    debug!("Query range from {} to {}", start_of_day, end_of_day);
    (start_of_day, end_of_day)
}

fn get_local_date_in_utc() -> String {
    Utc::now().date_naive().to_string()
}

/// Obtain the date n days ago in UTC
fn get_recall_date_in_utc(n: i64) -> String {
    let utc_now = Utc::now();
    let recall_date = utc_now.date_naive() - Duration::days(n);
    recall_date.to_string()
}

#[tauri::command]
pub fn get_recall_usage_duration_rs(n: i64) -> Result<HashMap<String, i64>, String> {
    let conn = init_db().map_err(|e| format!("DB error: {}", e))?;
    let local_date_in_utc = get_local_date_in_utc();
    let start_date_in_utc = get_recall_date_in_utc(n);
    let sql = format!(
        "SELECT * FROM {} WHERE date BETWEEN '{}' AND '{}'",
        TABLE::DAILY_USAGE_STATS,
        start_date_in_utc,
        local_date_in_utc
    );
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("SQL prepare error: {}", e))?;
    let mut rows = stmt
        .query([])
        .map_err(|e| format!("SQL query error: {}", e))?;
    let mut result = HashMap::new();
    while let Some(row) = rows.next().map_err(|e| format!("Rows next error: {}", e))? {
        let date = row
            .get::<_, String>(1)
            .map_err(|e| format!("Row get error: {}", e))?;
        let duration = row
            .get::<_, i64>(2)
            .map_err(|e| format!("Row get error: {}", e))?;
        result.insert(date, duration);
    }
    debug!("Recall daily usage: {}", jsonify(&result));
    Ok(result)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn init_logger() {
        env_logger::init();
    }

    #[test]
    fn test_get_app_usage_duration() {
        use crate::utils::db::init_db;
        let conn = init_db().expect("Error initializing database");
        update_daily_app_usage(&conn).expect("Error updating daily app usage");
    }

    #[test]
    fn test_get_daily_usage_duration() {
        use crate::utils::db::init_db;
        let conn = init_db().expect("Error initializing database");
        update_daily_usage_stats(&conn).expect("Error updating daily usage");
    }

    #[test]
    fn test_recall_daily_usage_duration() {
        let _ = get_recall_usage_duration_rs(7);
    }
}
