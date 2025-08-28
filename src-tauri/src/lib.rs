use chrono::Utc;
use rdev::{Event, EventType};
use rusqlite::params;
use std::thread;
use tauri::tray::TrayIconBuilder;

mod constants;
mod core;
mod utils;
use constants::db::TABLE;
use constants::window::{IGNORE_APP_LIST, WindowEvent};
use core::report::export_report;
use core::stats::{
    get_app_usage_duration_rs, get_recall_usage_duration_rs, update_daily_app_usage,
    update_daily_usage_stats,
};
use core::task::register_event_listener;
use core::task::register_scheduled_task;
use parking_lot::Mutex;
use tauri::AppHandle;
use utils::autostart::set_start_on_boot_rs;
use utils::db::{init_db, insert};
use utils::logging::Type;
use utils::window::{
    current_window, window_close, window_minimize, window_start_drag, window_toggle_always_on_top,
    window_toggle_maximize,
};

pub struct AppHandleManager {
    handle: Mutex<Option<AppHandle>>,
}

impl AppHandleManager {
    pub fn new() -> Self {
        Self {
            handle: Mutex::new(None),
        }
    }

    pub fn init(&self, handle: AppHandle) {
        let mut app_handler = self.handle.lock();
        if app_handler.is_none() {
            *app_handler = Some(handle);
            logging!(
                debug,
                Type::Setup,
                "{} initialized with app handle.",
                stringify!(Self)
            );
        }
    }

    /// Get the app handle
    pub fn get(&self) -> Option<AppHandle> {
        self.handle.lock().clone()
    }
}

// Global single instance
singleton_with_logging!(AppHandleManager, INSTANCE);

mod app_init {
    use super::*;

    pub fn setup_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
        builder
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_opener::init())
    }

    pub fn generate_handlers()
    -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
        tauri::generate_handler![
            // Dashboard statistics
            get_app_usage_duration_rs,
            get_recall_usage_duration_rs,
            // Settings
            set_start_on_boot_rs,
            // Export
            export_report,
            // Window event
            window_close,
            window_minimize,
            window_toggle_always_on_top,
            window_toggle_maximize,
            window_start_drag,
        ]
    }

    pub fn setup_tray_icon(
        app: &tauri::App,
    ) -> Result<tauri::tray::TrayIcon, Box<dyn std::error::Error>> {
        let _tray = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .build(app)?;
        Ok(_tray)
    }

    pub fn init_core(app_handle: &AppHandle) {
        let app_handle = app_handle.clone();
        AppHandleManager::global().init(app_handle);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
                // TO-DO: The calc algorithm is not right
                let cw = current_window();
                if pre_cw == cw {
                    return;
                };
                logging!(debug, Type::Window, "Current window: {}", cw);
                pre_cw = cw.clone();
                if IGNORE_APP_LIST.contains(&cw.as_str()) {
                    return;
                }
                let time_stamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let params = params![&time_stamp, &cw];
                insert(TABLE::APP_USAGE_LOGS, params).expect("Error inserting app usage log");
            }
            _ => {}
        });
    });

    let builder =
        app_init::setup_plugins(tauri::Builder::default().plugin(tauri_plugin_shell::init()))
            .invoke_handler(app_init::generate_handlers())
            .setup(|app| {
                let _tray = app_init::setup_tray_icon(app).expect("Error setting up tray icon");
                let app_handle = app.handle().clone();
                app_init::init_core(&app_handle);
                Ok(())
            });

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _event| {
        match _event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // api.prevent_exit();
                let time_stamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let params = params![&time_stamp, WindowEvent::EXITED];
                if let Err(e) = insert(TABLE::APP_USAGE_LOGS, params) {
                    eprintln!("Error inserting close log: {}", e);
                }
                logging!(
                    debug,
                    Type::Exit,
                    "App usage log inserted: [{} - {}]",
                    time_stamp,
                    WindowEvent::EXITED
                );
            }
            _ => {}
        }
    });
}
