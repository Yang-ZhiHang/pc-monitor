use crate::constants::report::ExportFmt;
use crate::core::stats::get_app_usage_duration_range;
use crate::logging;
use crate::utils::file::save_file_with_dialog;
use crate::utils::logging::Type;
use crate::utils::test::jsonify;
use std::fs;
use tera::{Context, Tera};

#[tauri::command]
pub fn export_report(start_date: &str, end_date: &str, format: &str) -> Result<(), String> {
    logging!(
        debug,
        Type::Report,
        false,
        "Exporting report from {} to {}, format: {}",
        start_date,
        end_date,
        format
    );
    let data = get_app_usage_duration_range(start_date, end_date)?;

    let content = match format {
        ExportFmt::JSON => jsonify(&data),
        ExportFmt::CSV => {
            let headers = vec!["date", "app_name", "duration"];
            let mut csv = headers.join(",") + "\n";
            for (date, apps) in data.iter() {
                for (app_name, duration) in apps.iter() {
                    let row = vec![date.to_string(), app_name.to_string(), duration.to_string()];
                    csv += &row.join(",");
                    csv += "\n";
                }
            }
            csv
        }
        ExportFmt::HTML => {
            let template = fs::read_to_string("templates/template_report_overview.html")
                .map_err(|e| format!("Template error: {}", e))?;
            let mut tera = Tera::default();
            tera.add_raw_template("report", &template)
                .map_err(|e| format!("Failed to add template: {}", e))?;

            let mut payload = Context::new();
            payload.insert("data", &data);

            tera.render("report", &payload)
                .map_err(|e| format!("Failed to render template: {}", e))?
        }
        _ => {
            return Err("Unsupported format".to_string());
        }
    };

    if let Err(e) = save_file_with_dialog(
        content.as_str(),
        format,
        format!("report_{}_to_{}", start_date, end_date).as_str(),
    ) {
        return Err(e.to_string());
    }
    Ok(())
}
