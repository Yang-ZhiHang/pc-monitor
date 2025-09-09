# PCMonitor

An app for monitoring and visualizing your PC's usage metrics over time.

🌏 README in
[**中文**](https://github.com/Yang-ZhiHang/pc-monitor/blob/master/docs/README.zh-CN.md) Provided

## Features

- Historical data visualization with charts and graphs
- Support for exporting data in HTML/CSV/JSON format
- Support language switching within English and Chinese

## Screenshots

The dashboard:

![dashboard](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/dashboard.jpg)

The export panel:

![export](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/export.jpg)

The settings panel:

![settings](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/setting.jpg)

## Next plan

- [ ] Customizable usage tracking frequency
- [ ] Refactor main dashboard
    - [ ] App usage rank
    - [ ] Change "Total this week" Card to "Total usage today"?

## Tech Stacks

- Tauri(Rust) + Vue3 + Vite
- SQLite
- Libraries: vue-i18n, vue-chartjs, dayjs