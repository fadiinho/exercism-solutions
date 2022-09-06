// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_level: &str;
    if let LogLevel::Error = level {
        log_level = "ERROR"
    } else if let LogLevel::Warning = level {
        log_level = "WARNING"
    } else if let LogLevel::Info = level {
        log_level = "INFO"
    } else if let LogLevel::Debug = level {
        log_level = "DEBUG"
    } else {
        log_level = "UNKNOWN"
    }

    format!("[{}]: {}", &log_level, &message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
