# PCMonitor

一个用于监控和可视化您的 PC 使用指标的应用程序。

## 功能

- 历史数据可视化，支持图表（柱状图和饼图）
- 支持以 HTML/CSV/JSON 格式导出数据
- 支持中英文切换

## 项目截图

首页仪表盘:

![dashboard](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/dashboard.zh-CN.jpg)

导出面板:

![export](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/export.zh-CN.jpg)

设置面板:

![settings](https://raw.githubusercontent.com/Yang-ZhiHang/pc-monitor/master/image/setting.zh-CN.jpg)

## 下一步计划

- [ ] 自定义数据刷新频率
- [ ] 首页仪表盘重构
    - [ ] 应用使用排行
    - [ ] 「本周总计」改为「今日总计」?

## 技术栈

- 整体使用 Tauri 框架进行开发，前端使用 Vue3 + Vite，后端使用 Rust
- 数据库：SQLite
- 前端用到的库: vue-i18n, vue-chartjs, dayjs