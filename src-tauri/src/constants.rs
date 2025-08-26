pub const APP_NAME: &str = "PCMonitor";

pub const DB_NAME: &str = "pc_monitor.db";
pub struct TABLE;

impl TABLE {
    pub const APP_USAGE_LOGS: &str = "app_usage_logs";
    pub const DAILY_APP_USAGE: &str = "daily_app_usage";
    pub const DAILY_USAGE_STATS: &str = "daily_usage_stats";
}

pub const IGNORE_APP_LIST: &[&str] = &[
    "Windows Shell Experience Host",
    "Windows Start Experience Host",
    "LockApp.exe",
    "Windows 资源管理器",
    "Windows Explorer",
    "任务管理器",
    "TaskManager",
    "Microsoft Edge WebView2",
];
