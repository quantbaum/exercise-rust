// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // unimplemented!("return a message for the given log level")
    match level{
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        _ => "".to_string()
    }
}
pub fn info(message: &str) -> String {
    // unimplemented!("return a message for info log level")
    format!("[INFO]: {}", message)
}
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
