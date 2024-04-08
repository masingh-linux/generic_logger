use std::{fs::File, io::Write};

use crate::stdio_logger::StdIOLogger;

pub const LOG_LEVEL_PANIC: u8 = 1;
pub const LOG_LEVEL_CRITICAL: u8 = 2;
pub const LOG_LEVEL_ERROR: u8 = 3;
pub const LOG_LEVEL_WARNING: u8 = 4;
pub const LOG_LEVEL_INFO: u8 = 5;
pub const LOG_LEVEL_DEBUG1: u8 = 6;
pub const LOG_LEVEL_DEBUG2: u8 = 7;
pub const LOG_LEVEL_DEBUG3: u8 = 8;
pub const LOG_LEVEL_DEBUG4: u8 = 9;

pub fn get_logger(level: u8) -> Box<dyn Logger>{
    let file = File::create("./.logger_config");
    match file {
        Ok(mut file) => {
           match file.write("To be Done in Later Version".as_bytes()) {
            Ok(_) => println!("Creted Logger config successfully. Update logger config to update logger params"),
            Err(e) => println!("Cannot create Logger config due to {}", e),
                   } 
        }
        Err(e) => println!("Cannot create Logger config due to {}", e),
    }
    
    Box::new(StdIOLogger {
        log_level: level,
    })
}
pub trait Logger {
    fn log_panic(&self, message: &str);
    fn log_critical(&self, message: &str);
    fn log_error(&self, message: &str);
    fn log_warning(&self, message: &str);
    fn log_info(&self, message: &str);
    fn log_debug1(&self, message: &str);
    fn log_debug2(&self, message: &str);
    fn log_debug3(&self, message: &str);
    fn log_debug4(&self, message: &str);
}