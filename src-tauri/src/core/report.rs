use crate::constants::db::TABLE;
use crate::constants::report::ExportFmt;
use crate::logging;
use crate::utils::db::init_db;
use crate::utils::file::save_file_with_dialog;
use crate::utils::logging::Type;
use serde_json::json;

#[tauri::command]
pub fn export_report(
    start_date: String,
    end_date: String,
    types: Vec<String>,
    format: String,
) -> Result<(), String> {
    logging!(
        debug,
        Type::Report,
        false,
        "Exporting report from {} to {}, types: {:?}, format: {}",
        start_date,
        end_date,
        types,
        format
    );
    let conn = init_db().map_err(|e| format!("DB error: {}", e))?;
    let mut result = serde_json::Map::new();

    // Query for data
    for t in types {
        match t.as_str() {
            "appUsage" => {
                // 查询 appUsage
                let sql = format!(
                    "SELECT app_name, total_usage, date FROM {} WHERE date BETWEEN ? AND ?",
                    TABLE::DAILY_APP_USAGE
                );
                let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
                let rows = stmt
                    .query_map([&start_date, &end_date], |row| {
                        Ok(json!({
                            "app_name": row.get::<_, String>(0)?,
                            "total_usage": row.get::<_, i64>(1)?,
                            "date": row.get::<_, String>(2)?
                        }))
                    })
                    .map_err(|e| e.to_string())?;
                let data: Vec<_> = rows.filter_map(Result::ok).collect();
                result.insert("appUsage".to_string(), json!(data));
            }
            _ => {}
        }
    }

    // Output in format
    let mut content = String::new();
    match format.as_str() {
        ExportFmt::JSON => {
            content = serde_json::to_string_pretty(&result).unwrap();
        }
        ExportFmt::CSV => {
            if let Some(arr) = result.values().next() {
                let arr = arr.as_array().unwrap();
                if arr.is_empty() {
                    return Ok(());
                }
                let headers: Vec<_> = arr[0].as_object().unwrap().keys().cloned().collect();
                let mut csv = headers.join(",") + "\n";
                for obj in arr {
                    let row: Vec<_> = headers
                        .iter()
                        .map(|h| obj.get(h).map_or("".to_string(), |v| v.to_string()))
                        .collect();
                    csv += &row.join(",");
                    csv += "\n";
                }
                content = csv;
            }
        }
        ExportFmt::HTML => {
            let mut html = String::from("<html><body>");
            for (key, arr) in &result {
                html += &format!("<h3>{}</h3><table border='1'><tr>", key);
                let arr = arr.as_array().unwrap();
                if !arr.is_empty() {
                    let headers: Vec<_> = arr[0].as_object().unwrap().keys().cloned().collect();
                    html += &headers
                        .iter()
                        .map(|h| format!("<th>{}</th>", h))
                        .collect::<String>();
                    html += "</tr>";
                    for obj in arr {
                        html += "<tr>";
                        html += &headers
                            .iter()
                            .map(|h| {
                                format!(
                                    "<td>{}</td>",
                                    obj.get(h).map_or("", |v| v.as_str().unwrap_or(""))
                                )
                            })
                            .collect::<String>();
                        html += "</tr>";
                    }
                }
                html += "</table>";
            }
            html += "</body></html>";
            content = html;
        }
        _ => {
            return Err("Unsupported format".to_string());
        }
    }

    if let Err(e) = save_file_with_dialog(
        content.as_str(),
        format.as_str(),
        format!("report_{}_to_{}", start_date, end_date).as_str(),
    ) {
        return Err(e.to_string());
    }
    Ok(())
}
