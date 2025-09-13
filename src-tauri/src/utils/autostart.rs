use crate::AppHandleManager;
use crate::{logging, utils::logging::Type};
use tauri_plugin_autostart::ManagerExt;

#[cfg(target_os = "windows")]
pub fn set_start_on_boot(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let app = AppHandleManager::global().get().unwrap();
    let app_launcher = app.autolaunch();

    let _ = match enable {
        true => app_launcher.enable(),
        false => app_launcher.disable(),
    };

    logging!(
        info,
        Type::Autostart,
        true,
        "The Startup is now {:?}",
        app_launcher.is_enabled()
    );
    Ok(())
}

#[tauri::command]
pub fn set_start_on_boot_rs(enable: bool) -> Result<(), String> {
    set_start_on_boot(enable).map_err(|e| e.to_string())
}
