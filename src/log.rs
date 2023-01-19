//! this logger will help us debug / log our kernel 

pub use log::{debug, error, info, trace, warn, LevelFilter};
use log::{Level, Metadata, Record};

use crate::serial_println;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            serial_println!("[{}] - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

/// initiate a logger static object
/// # Arguments
///  - `filter` the max level of our logger
///
/// ## Filter Levels
/// 1. Error
/// 2. Warn
/// 3. Info
/// 4. Debug
/// 5. Trace
pub fn init(filter: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(filter);
}
