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
            if let Some(arr) = result.get("appUsage") {
                let arr = arr.as_array().unwrap();
                // 构造 x 轴（日期），y 轴（使用时长），系列（应用名）
                let mut dates = vec![];
                let mut app_map = std::collections::BTreeMap::new();
                for obj in arr {
                    let app = obj["app_name"].as_str().unwrap_or("");
                    let date = obj["date"].as_str().unwrap_or("");
                    let usage = obj["total_usage"].as_i64().unwrap_or(0);
                    dates.push(date.to_string());
                    app_map
                        .entry(app.to_string())
                        .or_insert_with(Vec::new)
                        .push((date.to_string(), usage));
                }
                dates.sort();
                dates.dedup();

                // 构造 series
                let mut series = vec![];
                for (app, usage_vec) in app_map {
                    // 按日期补齐数据
                    let mut data = vec![];
                    for d in &dates {
                        let mut found = false;
                        for (date, usage) in &usage_vec {
                            if date == d {
                                data.push(*usage);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            data.push(0);
                        }
                    }
                    series.push(json!({
                        "name": app,
                        "type": "bar",
                        "stack": "total",
                        "data": data
                    }));
                }

                // 生成 HTML
                let html = format!(
                    r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>应用使用统计图表</title>
    <script src="https://cdn.jsdelivr.net/npm/echarts@5/dist/echarts.min.js"></script>
</head>
<body>
    <h2>应用使用统计（{start} ~ {end}）</h2>
    <div id="main" style="width: 1000px;height:600px;"></div>
    <script>
        var chartDom = document.getElementById('main');
        var myChart = echarts.init(chartDom);
        var option = {{
            tooltip: {{
                trigger: 'axis',
                axisPointer: {{ type: 'shadow' }}
            }},
            legend: {{ }},
            xAxis: {{
                type: 'category',
                data: {dates}
            }},
            yAxis: {{
                type: 'value'
            }},
            series: {series}
        }};
        myChart.setOption(option);
    </script>
</body>
</html>
"#,
                    start = start_date,
                    end = end_date,
                    dates = serde_json::to_string(&dates).unwrap(),
                    series = serde_json::to_string(&series).unwrap()
                );
                content = html;
            } else {
                content = "<html><body><h3>无数据</h3></body></html>".to_string();
            }
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
