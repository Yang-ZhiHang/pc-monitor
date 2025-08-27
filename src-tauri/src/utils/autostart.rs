use crate::constants::window::APP_NAME;
use crate::logging;
use crate::utils::logging::Type;
use std::env;
use winreg::{RegKey, enums::*};

#[cfg(target_os = "windows")]
pub fn set_start_on_boot(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?.to_str().unwrap().to_string();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;
    logging!(
        debug,
        Type::Autostart,
        false,
        "Setting start on boot to {}",
        enable
    );
    if enable {
        run.set_value(APP_NAME, &exe_path)?;
    } else {
        run.delete_value(APP_NAME)?;
    }
    Ok(())
}

#[tauri::command]
pub fn set_start_on_boot_rs(enable: bool) -> Result<(), String> {
    set_start_on_boot(enable).map_err(|e| e.to_string())
}
