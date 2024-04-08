#![allow(dead_code)]
pub(crate) mod stdio_logger;
pub mod logger;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let my_logger = logger::get_logger(logger::LOG_LEVEL_DEBUG4);
        println!("*************************************************************");
        my_logger.log_panic("panic...");
        my_logger.log_critical("critical....");
        my_logger.log_error("error...");
        my_logger.log_warning("warning...");
        my_logger.log_info("info...");
        my_logger.log_debug1("debug1......");
        my_logger.log_debug2("debug2....");
        my_logger.log_debug3("debug3.....");
        my_logger.log_debug4("debug4....");
        println!("*************************************************************");
        assert!(true);
    }
}
