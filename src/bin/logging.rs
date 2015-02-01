use log;
use std::str::FromStr;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _level: log::LogLevel, _module: &str) -> bool {
        true
    }

    fn log(&self, record: &log::LogRecord) {
        let module_path = record.location().module_path;
        if self.enabled(record.level(), module_path) {
            println!("{}|{}|{}", record.level(), module_path, record.args());
        }
    }
}

pub fn init_logging(level: &String) {
    let level_object: log::LogLevelFilter = match FromStr::from_str(level) {
        Some(l) => l,
        None => panic!("Illegal log level specified: '{}'", level),
    };

    match log::set_logger(|max_log_level| {
            max_log_level.set(level_object);
            Box::new(SimpleLogger)
        }) {
            Ok(_) => (),
            Err(_) => panic!(),
    };
}
