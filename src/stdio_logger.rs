use crate::logger::*;


pub(crate) struct StdIOLogger{
    pub(crate) log_level: u8,
}

impl Logger for StdIOLogger {
    fn log_panic(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_PANIC {
            println!("PANIC: {}", s);
        }
    }

    fn log_critical(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_CRITICAL {
            println!("CRIT: {}", s);
        }
    }

    fn log_error(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_ERROR {
            println!("ERR: {}", s);
        }
    }

    fn log_warning(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_WARNING {
            println!("WARN: {}", s);
        }
    }

    fn log_info(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_INFO {
            println!("INFO: {}", s);
        }
    }

    fn log_debug1(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG1 {
            println!("DBG1: {}", s);
        }
    }

    fn log_debug2(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG2 {
            println!("DBG2: {}", s);
        }
    }

    fn log_debug3(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG3 {
            println!("DBG3: {}", s);
        }
    }

    fn log_debug4(&mut self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG4 {
            println!("DBG4: {}", s);
        }
    }
}
