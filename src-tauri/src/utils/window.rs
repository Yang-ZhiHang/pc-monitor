use crate::AppHandleManager;
use crate::logging;
use crate::utils::logging::Type;
use std::path::Path;
use sysinfo::{Pid, System};
use tauri::Manager;

#[cfg(target_os = "windows")]
/// Get the window which is focused currently.
///
/// ## Returns
///
/// The name of the focused window.
pub fn current_window() -> String {
    use windows::{
        Win32::UI::WindowsAndMessaging::GetForegroundWindow,
        Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId,
    };
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut pid = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        let mut sys = System::new();
        sys.refresh_processes();
        if let Some(proc) = sys.process(Pid::from_u32(pid)) {
            let exe_opt = proc.exe();
            exe_opt
                .map(|exe| friendly_name_from_exe(&exe.to_path_buf()))
                .flatten()
                .unwrap_or_else(|| proc.name().to_string())
        } else {
            "unknown".into()
        }
    }
}

#[cfg(target_os = "windows")]
/// Get the friendly name (ProductName) from exe.
///
/// ## Params
///
/// - `exe`: The path of a executable file
///
/// ## Returns
///
/// The friendly name (ProductName) from a executable file (Option(String)).
///
/// ## Example
///
/// ```ignore
/// use super::*;
///
/// let path = Path::new(r"D:\app\Microsoft VS Code\Code.exe");
/// let friendly_name = friendly_name_from_exe(path);
/// assert_eq!(friendly_name.unwrap_or_default(), "Visual Studio Code");
/// ```
fn friendly_name_from_exe(exe: &Path) -> Option<String> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{
        Win32::Foundation::FALSE,
        Win32::Storage::FileSystem::{
            GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
        },
        core::PCWSTR,
    };

    let wide: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe {
        // 1. Obtain the size of version infomation in .exe
        let size = GetFileVersionInfoSizeW(PCWSTR(wide.as_ptr()), None);
        if size == 0 {
            return None;
        };

        // 2. Read version information into buffer according to size
        let mut buffer = vec![0u8; size as usize];
        if GetFileVersionInfoW(
            PCWSTR(wide.as_ptr()),
            0,
            size,
            buffer.as_mut_ptr() as *mut _,
        )
        .is_err()
        {
            return None;
        }

        // 3.1 Get the StringFileInfo block
        let mut info_ptr = std::ptr::null_mut();
        let mut info_len = 0;
        let sub_block = "\\VarFileInfo\\Translation"
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        if VerQueryValueW(
            buffer.as_ptr() as *const _,
            PCWSTR(sub_block.as_ptr()),
            &mut info_ptr,
            &mut info_len,
        ) == FALSE
            && info_len == 0
        {
            return None;
        }

        // 3.2 Take out the first language-codepage combo
        let trans = *(info_ptr as *const u32);
        let lang_cp = format!("{:04x}{:04x}", trans & 0xffff, (trans >> 16) & 0xffff);

        // 3.3 Spell out the Infomation Path
        let sub_block = format!("\\StringFileInfo\\{}\\FileDescription\0", lang_cp)
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let mut ptr = std::ptr::null_mut();
        let mut len = 0;

        // 4. Get friendly name from buffer
        if VerQueryValueW(
            buffer.as_ptr() as *const _,
            PCWSTR(sub_block.as_ptr()),
            &mut ptr,
            &mut len,
        ) == FALSE
            && len == 0
        {
            return None;
        }

        let slice = std::slice::from_raw_parts(ptr as *const u16, len as usize);
        return Some(
            String::from_utf16_lossy(slice)
                .trim_end_matches('\0')
                .into(),
        );
    }
}

#[tauri::command]
pub fn window_minimize() -> bool {
    WindowManager::minimize()
}

#[tauri::command]
pub fn window_toggle_maximize() -> bool {
    WindowManager::toggle_maximize()
}

#[tauri::command]
pub fn window_toggle_always_on_top() -> bool {
    WindowManager::toggle_always_on_top()
}

#[tauri::command]
pub fn window_close(hide: bool) {
    match WindowManager::get_main_window() {
        Some(window) => {
            if hide {
                match window.hide() {
                    Ok(_) => {
                        logging!(info, Type::Window, true, "Window is now hidden.");
                    }
                    Err(e) => {
                        logging!(error, Type::Window, true, "Hide window failed: {}", e);
                    }
                }
            } else {
                match window.close() {
                    Ok(_) => {
                        logging!(info, Type::Window, false, "Window closed.");
                    }
                    Err(e) => {
                        logging!(error, Type::Window, true, "Close window failed: {}", e);
                    }
                }
            }
        }
        None => {
            logging!(
                info,
                Type::Window,
                true,
                "Window does not exist, no need to close"
            );
        }
    }
}

#[tauri::command]
pub fn window_start_drag() -> bool {
    match WindowManager::get_main_window() {
        Some(window) => match window.start_dragging() {
            Ok(_) => {
                logging!(info, Type::Window, false, "Window drag started.");
                true
            }
            Err(e) => {
                logging!(warn, Type::Window, true, "Start drag failed: {}", e);
                false
            }
        },
        None => {
            logging!(
                info,
                Type::Window,
                true,
                "Window does not exist, no need to start drag"
            );
            false
        }
    }
}

pub struct WindowManager;

impl WindowManager {
    pub fn get_main_window() -> Option<tauri::WebviewWindow<tauri::Wry>> {
        AppHandleManager::global()
            .get()
            .and_then(|app_handle| app_handle.get_webview_window("main"))
    }

    pub fn minimize() -> bool {
        match Self::get_main_window() {
            Some(window) => match window.is_minimized() {
                Ok(_) => {
                    logging!(debug, Type::Window, false, "Window minimized");
                    let _ = window.minimize();
                    true
                }
                Err(e) => {
                    logging!(error, Type::Window, true, "Window minimize failed: {}", e);
                    false
                }
            },
            None => {
                logging!(warn, Type::Window, true, "Main window does not exist.");
                false
            }
        }
    }

    pub fn toggle_maximize() -> bool {
        match Self::get_main_window() {
            Some(window) => match window.is_maximized() {
                Ok(is_maximized) => {
                    if is_maximized {
                        let _ = window.unmaximize();
                    } else {
                        let _ = window.maximize();
                    }
                    logging!(debug, Type::Window, false, "Window maximize toggled.");
                    true
                }
                Err(e) => {
                    logging!(error, Type::Window, true, "Window minimize failed: {}", e);
                    false
                }
            },
            None => {
                logging!(warn, Type::Window, true, "Main window does not exist.");
                false
            }
        }
    }

    pub fn toggle_always_on_top() -> bool {
        match Self::get_main_window() {
            Some(window) => match window.is_always_on_top() {
                Ok(is_always_on_top) => {
                    if is_always_on_top {
                        let _ = window.set_always_on_top(false);
                    } else {
                        let _ = window.set_always_on_top(true);
                    }
                    true
                }
                Err(e) => {
                    logging!(error, Type::Window, true, "Window operation failed: {}", e);
                    false
                }
            },
            None => {
                logging!(warn, Type::Window, true, "Main window does not exist.");
                false
            }
        }
    }
}
