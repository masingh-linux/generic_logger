use std::{fs::File, io::Write};

use crate::stdio_logger::StdIOLogger;
use crate::fileio_logger::FileIOLogger;


pub const LOG_LEVEL_PANIC: u8 = 1;
pub const LOG_LEVEL_CRITICAL: u8 = 2;
pub const LOG_LEVEL_ERROR: u8 = 3;
pub const LOG_LEVEL_WARNING: u8 = 4;
pub const LOG_LEVEL_INFO: u8 = 5;
pub const LOG_LEVEL_DEBUG1: u8 = 6;
pub const LOG_LEVEL_DEBUG2: u8 = 7;
pub const LOG_LEVEL_DEBUG3: u8 = 8;
pub const LOG_LEVEL_DEBUG4: u8 = 9;

pub enum LoggerType {
    StdIO,
    FileIO,
}

pub fn get_logger_type(logger_type: LoggerType, level: u8) -> Box<dyn Logger> {
    match logger_type {
        LoggerType::StdIO => Box::new(StdIOLogger {
            log_level: level,
        }),
        LoggerType::FileIO => Box::new(FileIOLogger::new("logs.txt", level)),
    }
}

pub trait Logger {
    fn log_panic(&mut self, message: &str);
    fn log_critical(&mut self, message: &str);
    fn log_error(&mut self, message: &str);
    fn log_warning(&mut self, message: &str);
    fn log_info(&mut self, message: &str);
    fn log_debug1(&mut self, message: &str);
    fn log_debug2(&mut self, message: &str);
    fn log_debug3(&mut self, message: &str);
    fn log_debug4(&mut self, message: &str);
}