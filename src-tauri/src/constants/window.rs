pub const APP_NAME: &str = "PCMonitor";

pub struct WindowEvent;

impl WindowEvent {
    pub const EXITED: &str = "Application Exited";
    pub const LOCKED: &str = "LockApp.exe";
}

pub const W_IGNORE_APP_LIST: &[&str] = &[
    "Windows Shell Experience Host",
    "Windows Start Experience Host",
    "Microsoft Edge WebView2",
];

pub const R_IGNORE_APP_LIST: &[&str] = &[
    "LockApp.exe",
    "Windows 资源管理器",
    "Windows Explorer",
    "任务管理器",
    "TaskManager",
    WindowEvent::LOCKED,
    WindowEvent::EXITED,
];
