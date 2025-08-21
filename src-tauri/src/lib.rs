use rdev::{Event, EventType, listen};
use std::thread;
use tauri::tray::TrayIconBuilder;
mod core;
mod utils;
mod constants;
use core::db::init_db;
use utils::window::current_window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_db().expect("Error initializing database");
    thread::spawn(|| {
        listen(|evt: Event| match evt.event_type {
            EventType::KeyRelease(_) | EventType::ButtonRelease(_) => {
                let ca = current_window();
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
