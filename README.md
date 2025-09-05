# PCMonitor

An app for monitoring and visualizing your PC's usage metrics over time.

## Features

- Historical data visualization with charts and graphs
- Support for exporting data in HTML/CSV/JSON format
- Support language switching within English and Chinese

## Next plan

- [x] Title bar + Github link
- [x] Add R_IGNORE_LIST in get_app_usage_duration_xxx
- [x] Singleton pattern needed in init_db()
- [x] Ensure the usage of &str and String, optimize the code
- [ ] i18n in rust backend
- [ ] Customizable usage tracking frequency
- [ ] A more user-friendly export panel (sync with the style of export template?)
    - [ ] modular panel: Export fmt、data summary、fast choice for date range
- [ ] Refactor main dashboard
    - [ ] App usage rank
    - [ ] [Total this week] -> [Total usage today]

## Tech Stacks

- Tauri: Vue3 + Vite, Rust
- SQLite
- Libraries: vue-i18n, vue-chartjs, dayjs