use rdev::{Event, EventType, listen};
use rusqlite::params;
use std::thread;
use tauri::tray::TrayIconBuilder;

mod constants;
mod core;
mod utils;
use constants::TABLE;
use core::schedule::register_scheduled_task;
use utils::db::{init_db, insert};
use utils::window::current_window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _conn = init_db().expect("Error initializing database");

    // The thread where scheduled tasks and event listening run
    thread::spawn(|| {
        use tokio::time::Duration;
        register_scheduled_task(
            || {
                println!("2");
            },
            Duration::from_secs(2),
        );
        listen(|evt: Event| match evt.event_type {
            EventType::ButtonRelease(_) => {
                let ca = current_window();
                let time_stamp = chrono::Utc::now();
                let params = params![&ca, time_stamp.to_string()];
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
