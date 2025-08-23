use crate::constants::TABLE;
use chrono::{ Local, TimeZone, Utc};
use log::debug;
use rusqlite::{Connection, params};
use std::collections::HashMap;

/// Update the daily usage statistics for a specific application (data derived from app usage logs).
pub fn update_daily_app_usage(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 1. Obtain today's date (yyyy-mm-dd)
    let today = chrono::Utc::now().date_naive().to_string();

    // 2. Update usage duration for each app
    let usage_map = get_app_usage_duration(&conn)?;
    for (key, val) in &usage_map {
        let sql = format!(
            "INSERT INTO {} (date, app_name, total_usage) VALUES (?, ?, ?) ON CONFLICT(date, app_name) DO UPDATE SET total_usage = ?",
            TABLE::DAILY_APP_USAGE
        );
        let mut stmt = conn.prepare(&sql)?;
        stmt.execute(params![today, key, val, val])?;
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
pub fn get_app_usage_duration(conn: &Connection) -> Result<HashMap<String, i64>, rusqlite::Error> {
    let (start_of_day, end_of_day) = get_local_day_start_end_in_utc();

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
    let mut record: HashMap<String, i64> = HashMap::new();
    while let Some(row) = rows.next()? {
        let cur_time = row.get::<_, String>(1)?;
        let cur_name = row.get::<_, String>(2)?;
        if let (Ok(pre_dt), Ok(cur_dt)) = (
            chrono::NaiveDateTime::parse_from_str(&pre_time.as_ref().unwrap(), "%Y-%m-%d %H:%M:%S"),
            chrono::NaiveDateTime::parse_from_str(&cur_time, "%Y-%m-%d %H:%M:%S"),
        ) {
            let duration = cur_dt.signed_duration_since(pre_dt).num_seconds();
            record
                .entry(pre_name.as_ref().unwrap().clone())
                .and_modify(|v| *v += duration)
                .or_insert(duration);
        }
        pre_time = Some(cur_time);
        pre_name = Some(cur_name);
    }
    debug!("total_usage: {:?}", record);
    Ok(record)
}

/// Get all app usage durations that occurred today
pub fn get_daily_usage_duration(conn: &Connection) -> Result<i64, rusqlite::Error> {
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

/// Obtain the starting and ending times of the day to which this moment belongs
fn get_local_day_start_end_in_utc() -> (String, String) {
    let local = Local::now();
    let date = local.date_naive();
    let local_start = date.and_hms_opt(0, 0, 0).unwrap();
    let local_end = date.and_hms_opt(23, 59, 59).unwrap();
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
    println!("Query range from {} to {}", start_of_day, end_of_day);
    (start_of_day, end_of_day)
}

fn get_local_date_in_utc() -> String {
    Utc::now().date_naive().to_string()
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
