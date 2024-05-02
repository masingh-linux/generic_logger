use crate::logger::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub(crate) struct FileIOLogger {
    pub(crate) log_level: u8,
    // file_path: String,
    file: File,
}

impl FileIOLogger {
    pub(crate) fn new(file_path: &str, log_level: u8) -> Self {
        let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(file_path).expect("Failed to open or create log file");
        Self { log_level, file }
    }

    fn write_log(&mut self, message: &str) {
        writeln!(&mut self.file, "{}", message).expect("Failed to write to log file");
    }
}

impl Logger for FileIOLogger {
    fn log_panic(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_critical(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_error(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_warning(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_info(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_debug1(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_debug2(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_debug3(&mut self, s: &str) {
        self.write_log(s);
    }

    fn log_debug4(&mut self, s: &str) {
        self.write_log(s);
    }
}
