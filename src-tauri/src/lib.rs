use rdev::{Event, EventType};
use rusqlite::params;
use std::collections::HashMap;
use std::thread;
use tauri::tray::TrayIconBuilder;

mod constants;
mod core;
mod utils;
use constants::{IGNORE_APP_LIST, TABLE};
use core::stats::{
    get_app_usage_duration, get_recall_usage_duration_rs, update_daily_app_usage,
    update_daily_usage_stats,
};
use core::task::register_scheduled_task;
use utils::db::{init_db, insert};
use utils::window::current_window;

use crate::core::task::register_event_listener;

#[tauri::command]
fn get_app_usage_duration_rs(local_date: String) -> Result<HashMap<String, i64>, String> {
    use chrono::NaiveDate;
    let conn = init_db().map_err(|e| format!("DB error: {}", e))?;
    // convert yyyy-mm-dd String to NaiveDate
    let local_date = NaiveDate::parse_from_str(&local_date, "%Y-%m-%d")
        .map_err(|e| format!("Date parse error: {}", e))?;
    get_app_usage_duration(&conn, local_date).map_err(|e| format!("Query error: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // env_logger::init();

    // The thread where scheduled tasks and event listening run
    thread::spawn(|| {
        use tokio::time::Duration;
        register_scheduled_task(
            "update_daily_app_usage",
            || {
                let conn = init_db().expect("Error initializing database");
                update_daily_app_usage(&conn).expect("Error updating daily app usage");
            },
            Duration::from_secs(600),
        );
        register_scheduled_task(
            "update_daily_usage_stats",
            || {
                let conn = init_db().expect("Error initializing database");
                update_daily_usage_stats(&conn).expect("Error updating daily usage stats");
            },
            Duration::from_secs(600),
        );
        let mut pre_cw: String = "".to_string();
        register_event_listener("move_click", move |evt: Event| match evt.event_type {
            EventType::ButtonRelease(_) | EventType::KeyRelease(_) => {
                let cw = current_window();
                if pre_cw == cw || IGNORE_APP_LIST.contains(&cw.as_str()) {
                    return;
                }
                pre_cw = cw.clone();
                let time_stamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let params = params![&time_stamp, &cw];
                insert(TABLE::APP_USAGE_LOGS, params).expect("Error inserting app usage log");
                println!("Current window: {}", cw);
            }
            _ => {}
        });
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_usage_duration_rs,
            get_recall_usage_duration_rs,
        ])
        .setup(|app| {
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
