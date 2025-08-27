use crate::logging;
use crate::utils::logging::Type;
use std::path::Path;
use sysinfo::{Pid, System};
use tauri::Window;

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

    logging!(
        debug,
        Type::Window,
        false,
        "Target exe path: {}",
        exe.display()
    );
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
            logging!(
                debug,
                Type::Window,
                false,
                "VerQueryValueW(Translation) failed: {}",
                windows::core::Error::from_win32()
            );
            return None;
        }

        // 3.2 Take out the first language-codepage combo
        let trans = *(info_ptr as *const u32);
        let lang_cp = format!("{:04x}{:04x}", trans & 0xffff, (trans >> 16) & 0xffff);
        logging!(
            debug,
            Type::Window,
            false,
            "Detected lang-codepage: {}",
            lang_cp
        );

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
            logging!(
                debug,
                Type::Window,
                false,
                "VerQueryValueW(FileDescription) failed or empty"
            );
            return None;
        }

        let slice = std::slice::from_raw_parts(ptr as *const u16, len as usize);
        logging!(
            debug,
            Type::Window,
            false,
            "Got FileDescription: {}",
            String::from_utf16_lossy(slice)
        );
        return Some(
            String::from_utf16_lossy(slice)
                .trim_end_matches('\0')
                .into(),
        );
    }
}

#[tauri::command]
pub fn window_minimize(window: Window) {
    let _ = window.minimize();
}

#[tauri::command]
pub fn window_toggle_maximize(window: Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
pub fn window_toggle_always_on_top(window: Window) {
    let is_always_on_top = window.is_always_on_top().unwrap_or(false);
    let _ = window.set_always_on_top(!is_always_on_top);
}

#[tauri::command]
pub fn window_close(window: Window) {
    let _ = window.close();
}

#[tauri::command]
pub fn window_start_drag(window: Window) {
    let _ = window.start_dragging();
}
