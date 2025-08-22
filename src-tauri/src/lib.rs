use rdev::{Event, EventType, listen};
use rusqlite::params;
use std::thread;
use tauri::tray::TrayIconBuilder;

mod core;
mod constants;
mod utils;
use constants::TABLE;
use core::schedule::register_scheduled_task;
use core::stats::{update_daily_app_usage, update_daily_usage_stats};
use utils::db::{init_db, insert};
use utils::window::current_window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

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
        listen(|evt: Event| match evt.event_type {
            EventType::ButtonRelease(_) => {
                let ca = current_window();
                let time_stamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let params = params![&time_stamp, &ca];
                insert(TABLE::APP_USAGE_LOGS, params).expect("Error inserting app usage log");
                println!("Current window: {}", ca);
            }
            _ => {}
        })
        .expect("Error listening for events");
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
