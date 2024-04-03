pub const LOG_LEVEL_PANIC: u8 = 1;
pub const LOG_LEVEL_CRITICAL: u8 = 2;
pub const LOG_LEVEL_ERROR: u8 = 3;
pub const LOG_LEVEL_WARNING: u8 = 4;
pub const LOG_LEVEL_INFO: u8 = 5;
pub const LOG_LEVEL_DEBUG1: u8 = 6;
pub const LOG_LEVEL_DEBUG2: u8 = 7;
pub const LOG_LEVEL_DEBUG3: u8 = 8;
pub const LOG_LEVEL_DEBUG4: u8 = 9;

pub struct Logger {
    log_level: u8,
}

impl Logger {
    pub fn new(ll: u8) -> Logger {
        Logger {
            log_level: ll,
        }
    }

    pub fn log_panic(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_PANIC {
            println!("{}", s);
        }
    }

    pub fn log_critical(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_CRITICAL {
            println!("{}", s);
        }
    }

    pub fn log_error(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_ERROR {
            println!("{}", s);
        }
    }

    pub fn log_warning(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_WARNING {
            println!("{}", s);
        }
    }

    pub fn log_info(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_INFO {
            println!("{}", s);
        }
    }

    pub fn log_debug1(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG1 {
            println!("{}", s);
        }
    }

    pub fn log_debug2(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG2 {
            println!("{}", s);
        }
    }

    pub fn log_debug3(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG3 {
            println!("{}", s);
        }
    }

    pub fn log_debug4(&self, s: &str) {
        if self.log_level >= LOG_LEVEL_DEBUG4 {
            println!("{}", s);
        }
    }
}
