use colored::*;
use std::{cell::RefCell, sync::Mutex, thread::current};

/// Log level flag
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

struct Logger {
    level: Level,
    file: Option<String>,
}

impl Logger {
    fn new() -> Self {
        Self {
            level: Level::Info,
            file: None,
        }
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn set_file(&mut self, file: Option<String>) {
        self.file = file;
    }

    fn log(&self, level: Level, owner: &str, message: &str) {
        use chrono::*;
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        if level >= self.level {
            if let Some(ref file) = self.file {
                let result = match level {
                    Level::Debug => format!("{} [DEBUG] {:>60} |: {}\n", timestamp, owner, message),
                    Level::Info => format!("{} [INFO]  {:>60} |: {}\n", timestamp, owner, message),
                    Level::Warn => format!("{} [WARN]  {:>60} |: {}\n", timestamp, owner, message),
                    Level::Error => format!("{} [ERROR] {:>60} |: {}\n", timestamp, owner, message),
                };
                use std::fs::OpenOptions;
                use std::io::Write;
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .unwrap();
                file.write_all(result.as_bytes()).unwrap();
            } else {
                let result = match level {
                    Level::Debug => format!(
                        "{} {:<7} {:>60} |: {}",
                        timestamp,
                        "[DEBUG]".green().italic().underline(),
                        owner,
                        message
                    )
                    .green(),
                    Level::Info => format!(
                        "{} {:<7} {:>60} |: {}",
                        timestamp,
                        "[INFO]".blue(),
                        owner,
                        message
                    )
                    .blue(),
                    Level::Warn => format!(
                        "{} {:<7} {:>60} |: {}",
                        timestamp,
                        "[WARN]".yellow().bold(),
                        owner,
                        message
                    )
                    .yellow(),
                    Level::Error => format!(
                        "{} {:<7} {:>60} |: {}",
                        timestamp,
                        "[ERROR]".on_red().bold().underline(),
                        owner,
                        message
                    )
                    .red(),
                };
                if level == Level::Error {
                    eprintln!("{}", result);
                } else {
                    println!("{}", result);
                }
            }
        }
    }
}
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER_INIT: Mutex<Logger> = Mutex::new(Logger::new());
}

thread_local! {
    static THREAD_NAME: RefCell<String> = RefCell::new(format!("{:?}",current().id()));
}

/// settings functions for log.
pub struct Log;

impl Log {
    /// Set log level.
    /// By default, the log level is `Info`.
    pub fn set_level(level: Level) {
        let mut logger = LOGGER_INIT.lock().unwrap();
        logger.set_level(level);
    }

    /// Set log output file.
    /// By default, logs are output to the console.
    pub fn set_file(file: Option<String>) {
        let mut logger = LOGGER_INIT.lock().unwrap();
        logger.set_file(file);
    }

    /// Set the thread name displayed in the log output of the thread calling this function.
    /// By default, the thread name is `ThreadId(id)`.
    pub fn set_current_thread_name(name: &str) {
        THREAD_NAME.with_borrow_mut(|s| *s = String::from(name));
    }
}

/// Log output function.
pub fn log(level: Level, owner: &str, message: &str) {
    let logger = LOGGER_INIT.lock().unwrap();
    let owner = format!("{} @{:<20}", owner, THREAD_NAME.with_borrow(|s| s.clone()));
    logger.log(level, &owner, message);
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    (Self, $($arg:tt)*) => {
        $crate::debug!(std::any::type_name::<Self>(), $($arg)*);
    };
    (self, $($arg:tt)*) => {
        $crate::debug!(&format!("{}:{}:{}", file!(), line!(), column!()), $($arg)*);
    };
    ($owner:expr, $($arg:tt)*) => {
        $crate::log($crate::Level::Debug, $owner, &format_args!($($arg)*).to_string());
    };
    ($($arg:tt)*) => {
        $crate::debug!(self, $($arg)*);
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($owner:expr, $($arg:tt)*) => {};
    (Self, $($arg:tt)*) => {};
    (self, $($arg:tt)*) => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! info {
    (Self, $($arg:tt)*) => {
        $crate::info!(std::any::type_name::<Self>(), $($arg)*);
    };
    (self, $($arg:tt)*) => {
        $crate::info!(&format!("{}:{}:{}", file!(), line!(), column!()), $($arg)*);
    };
    ($owner:expr, $($arg:tt)*) => {
        $crate::log($crate::Level::Info, $owner, &format_args!($($arg)*).to_string());
    };
    ($($arg:tt)*) => {
        $crate::info!(self, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    (Self, $($arg:tt)*) => {
        $crate::warn!(std::any::type_name::<Self>(), $($arg)*);
    };
    (self, $($arg:tt)*) => {
        $crate::warn!(&format!("{}:{}:{}", file!(), line!(), column!()), $($arg)*);
    };
    ($owner:expr, $($arg:tt)*) => {
        $crate::log($crate::Level::Warn, $owner, &format_args!($($arg)*).to_string());
    };
    ($($arg:tt)*) => {
        $crate::warn!(self, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    (Self, $($arg:tt)*) => {
        $crate::error!(std::any::type_name::<Self>(), $($arg)*);
    };
    (self, $($arg:tt)*) => {
        $crate::error!(&format!("{}:{}:{}", file!(), line!(), column!()), $($arg)*);
    };
    ($owner:expr, $($arg:tt)*) => {
        $crate::log($crate::Level::Error, $owner, &format_args!($($arg)*).to_string());
    };
    ($($arg:tt)*) => {
        $crate::error!(self, $($arg)*);
    };
}
