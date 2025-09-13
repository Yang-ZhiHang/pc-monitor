use super::file::get_exe_path;
use chrono::Local;
use simplelog::*;
use std::fs::File;
use std::{env, fmt};
use time::macros::format_description;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = get_exe_path()?;
    let log_path = exe_path
        .parent()
        .ok_or("Failed to get exe parent dir")?
        .join(format!(
            "pc-monitor_{}.log",
            Local::now().format("%Y-%m-%d_%H-%M-%S")
        ));
    let log_file = File::create(&log_path)?;

    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .set_time_offset_to_local()
        .unwrap()
        .build();

    // Set log level from RUST_LOG env variable, default to Info
    // use `$env:RUST_LOG="debug";` in powershell for debug level
    let env_log_level = env::var("RUST_LOG")
        .map(|x| match x.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        })
        .unwrap_or(LevelFilter::Info);

    CombinedLogger::init(vec![
        TermLogger::new(
            env_log_level,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])?;
    Ok(())
}

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
