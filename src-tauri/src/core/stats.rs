use crate::constants::window::R_IGNORE_APP_LIST;
use crate::logging;
use crate::utils::logging::Type;
use crate::utils::test::jsonify;
use crate::{constants::db::TABLE, utils::db::DbManager};
use chrono::{Days, Local, NaiveDate, TimeZone, Utc};
use rusqlite::{Connection, params};
use std::collections::HashMap;

/// Update the daily usage statistics for a specific application (data derived from app usage logs).
pub fn update_daily_app_usage(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 1. Obtain today's date (yyyy-mm-dd)
    let utc_today = chrono::Utc::now().date_naive().to_string();

    // 2. Update usage duration for each app
    let local_today = Local::now().date_naive();
    let usage_map = collect_app_usage_duration(&conn, local_today)?;
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
fn collect_app_usage_duration(
    conn: &Connection,
    local_date: NaiveDate,
) -> Result<HashMap<String, i64>, rusqlite::Error> {
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
    while let Some(row) = rows.next()? {
        pre_time = Some(row.get::<_, String>(1)?);
        pre_name = Some(row.get::<_, String>(2)?);
        logging!(
            debug,
            Type::Statistics,
            false,
            "First log entry: time = {}, app = {}",
            pre_time.as_ref().unwrap(),
            pre_name.as_ref().unwrap()
        );

        // In the case that the result of query is none
        if pre_name.is_none() {
            return Ok(HashMap::new());
        }

        // Don't display applications which R_IGNORE_APP_LIST contains
        if R_IGNORE_APP_LIST.contains(&pre_name.as_ref().unwrap().as_str()) {
            continue;
        }

        break;
    }

    // TO-DO: if PC is sleepping, the sleep event may not be recorded
    // Traverse from the second row onwards
    let mut result: HashMap<String, i64> = HashMap::new();
    while let Some(row) = rows.next()? {
        let cur_time = row.get::<_, String>(1)?;
        let cur_name = row.get::<_, String>(2)?;

        if R_IGNORE_APP_LIST.contains(&pre_name.as_ref().unwrap().as_str()) {
            pre_time = Some(cur_time);
            pre_name = Some(cur_name);
            continue;
        }

        if let (Ok(pre_dt), Ok(cur_dt)) = (
            chrono::NaiveDateTime::parse_from_str(&pre_time.as_ref().unwrap(), "%Y-%m-%d %H:%M:%S"),
            chrono::NaiveDateTime::parse_from_str(&cur_time, "%Y-%m-%d %H:%M:%S"),
        ) {
            logging!(
                debug,
                Type::Statistics,
                false,
                "Comparing log entries: pre = {} - {}, cur = {} - {}",
                pre_time.as_ref().unwrap(),
                pre_name.as_ref().unwrap(),
                cur_time,
                cur_name
            );
            let duration = cur_dt.signed_duration_since(pre_dt).num_seconds();
            result
                .entry(pre_name.as_ref().unwrap().clone())
                .and_modify(|v| *v += duration)
                .or_insert(duration);
        }
        pre_time = Some(cur_time);
        pre_name = Some(cur_name);
    }
    logging!(
        debug,
        Type::Statistics,
        false,
        "Total app usage duration: {}",
        jsonify(&result)
    );
    Ok(result)
}

/// Get all app usage durations within specific range
///
/// ## Params
///
/// The date requires utc time
fn get_app_usage_duration(
    conn: &Connection,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<HashMap<String, HashMap<String, u64>>, rusqlite::Error> {
    logging!(
        debug,
        Type::Statistics,
        false,
        "The time range from {} to {}",
        start_date,
        end_date
    );

    let sql = format!(
        "SELECT * FROM {} WHERE date BETWEEN '{}' AND '{}'",
        TABLE::DAILY_APP_USAGE,
        start_date,
        end_date
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;

    let mut result: HashMap<String, HashMap<String, u64>> = HashMap::new();

    while let Some(row) = rows.next()? {
        let date = row.get::<_, String>(1)?;
        let app_name = row.get::<_, String>(2)?;
        let total_usage = row.get::<_, u64>(3)?;
        if R_IGNORE_APP_LIST.contains(&app_name.as_str()) {
            continue;
        }
        result
            .entry(date)
            .or_insert_with(HashMap::new)
            .insert(app_name, total_usage);
    }

    Ok(result)
}

/// Get all app usage durations that occurred today
fn get_daily_usage_duration(conn: &Connection) -> Result<u64, rusqlite::Error> {
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
        total_duration += row.get::<_, u64>(3)?;
    }

    Ok(total_duration)
}

/// Obtain the starting and ending times of the { local_date }
fn get_local_day_start_end_in_utc(local_date: NaiveDate) -> (String, String) {
    let local_start = local_date.and_hms_opt(0, 0, 0).unwrap();
    let local_end = local_date.and_hms_opt(23, 59, 59).unwrap();
    // Convert to UTC
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
    logging!(
        debug,
        Type::Statistics,
        false,
        "Query range from {} to {}",
        start_of_day,
        end_of_day
    );
    (start_of_day, end_of_day)
}

fn get_local_date_in_utc() -> NaiveDate {
    Utc::now().date_naive()
}

/// Obtain the date n days ago in UTC
fn get_recall_date_in_utc(start: NaiveDate, n: u64) -> NaiveDate {
    start - Days::new(n)
}

#[tauri::command]
pub fn get_app_usage_duration_last_n_days(
    n: u64,
) -> Result<HashMap<String, HashMap<String, u64>>, String> {
    let conn = DbManager::global().get().lock();
    let now_date = Utc::now().date_naive();
    let start_date = now_date - Days::new(n);
    match get_app_usage_duration(&conn, start_date, now_date) {
        Ok(mp) => Ok(mp),
        Err(e) => Err(format!("Error occured: {}", e)),
    }
}

#[tauri::command]
pub fn get_app_usage_duration_range(
    start_date: &str,
    end_date: &str,
) -> Result<HashMap<String, HashMap<String, u64>>, String> {
    let conn = DbManager::global().get().lock();
    let utc_start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let utc_end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();
    match get_app_usage_duration(&conn, utc_start_date, utc_end_date) {
        Ok(mp) => Ok(mp),
        Err(e) => Err(format!("Error occured: {}", e)),
    }
}

#[tauri::command]
pub fn get_daily_usage_duration_last_n_days(n: u64) -> Result<HashMap<String, u64>, String> {
    let conn = DbManager::global().get().lock();
    let local_date_in_utc = get_local_date_in_utc();
    let start_date_in_utc = get_recall_date_in_utc(local_date_in_utc, n);
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
            .get::<_, u64>(2)
            .map_err(|e| format!("Row get error: {}", e))?;
        result.insert(date, duration);
    }
    logging!(
        debug,
        Type::Statistics,
        false,
        "Recall daily usage: {}",
        jsonify(&result)
    );
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
    pub fn test_get_app_usage_duration_last_n_days() {
        let mp = get_app_usage_duration_last_n_days(7).expect("msg");
        logging!(
            debug,
            Type::Statistics,
            false,
            "The get_app_usage_duration_last_n_days result is {}",
            jsonify(&mp)
        );
    }

    #[test]
    pub fn test_get_app_usage_duration_range() {
        let mp = get_app_usage_duration_range("2025-08-29", "2025-9-02").expect("msg");
        logging!(
            debug,
            Type::Statistics,
            false,
            "The get_app_usage_duration_range result is {}",
            jsonify(&mp)
        );
    }
}
