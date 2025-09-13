use crate::AppHandleManager;
use std::{env, path::PathBuf};
use tauri_plugin_dialog::DialogExt;

pub fn get_exe_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    Ok(exe_path)
}

pub fn save_file_with_dialog(
    content: &str,
    fmt: &str,
    default_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = AppHandleManager::global()
        .get()
        .expect("App not initialized");
    let file_path = app
        .dialog()
        .file()
        .add_filter(default_name, &[fmt])
        .blocking_save_file();

    match file_path {
        Some(p) => {
            let path_buf = p.as_path().unwrap();
            std::fs::write(path_buf, content)?;
            Ok(())
        }
        None => Err("File save canceled".into()),
    }
}
