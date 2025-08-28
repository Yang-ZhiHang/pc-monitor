pub const APP_NAME: &str = "PCMonitor";

pub struct WindowEvent;

impl WindowEvent {
    pub const EXITED: &str = "Application Exited";
    pub const LOCKED: &str = "LockApp.exe";
}

pub const IGNORE_APP_LIST: &[&str] = &[
    "Windows Shell Experience Host",
    "Windows Start Experience Host",
    // "LockApp.exe",
    // "Windows 资源管理器",
    // "Windows Explorer",
    // "任务管理器",
    // "TaskManager",
    "Microsoft Edge WebView2",
    WindowEvent::EXITED,
];
