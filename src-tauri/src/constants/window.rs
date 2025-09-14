pub const APP_NAME: &str = "pc-monitor";

pub struct WindowEvent;

impl WindowEvent {
    pub const EXITED: &str = "Application Exited";
    pub const LOCKED: &str = "LockApp.exe";
}

pub const W_IGNORE_APP_LIST: &[&str] = &[
    "Windows Shell Experience Host",
    "Windows Start Experience Host",
];

pub const R_IGNORE_APP_LIST: &[&str] = &[
    "LockApp.exe",
    "Windows 资源管理器",
    "Windows Explorer",
    "Windows Terminal Host",
    "ShellHost",
    "任务管理器",
    "TaskManager",
    WindowEvent::LOCKED,
    WindowEvent::EXITED,
];
