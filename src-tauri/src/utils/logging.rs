use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Setup,
    Exit,
    Statistics,
    Task,
    Autostart,
    Window,
    Report,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Setup => write!(f, "[Setup]"),
            Type::Exit => write!(f, "[Exit]"),
            Type::Statistics => write!(f, "[Statistics]"),
            Type::Task => write!(f, "[Task]"),
            Type::Autostart => write!(f, "[Autostart]"),
            Type::Window => write!(f, "[Window]"),
            Type::Report => write!(f, "[Report]"),
        }
    }
}

#[macro_export]
macro_rules! logging {
    ($level:ident, $type:expr, true, $($arg:tt)*) => {
        println!("{} {}", $type, format_args!($($arg)*));
        log::$level!("{} {}", $type, format_args!($($arg)*));
    };
    ($level:ident, $type:expr, false, $($arg:tt)*) => {
        log::$level!("{} {}", $type, format_args!($($arg)*));
    };
    ($level:ident, $type:expr, $($arg:tt)*) => {
        println!("{} {}", $type, format_args!($($arg)*));
        log::$level!("{} {}", $type, format_args!($($arg)*));
    };

}
