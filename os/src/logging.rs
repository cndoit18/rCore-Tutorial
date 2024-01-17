use core::str::FromStr;

use log::{Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let color = match record.level() {
            log::Level::Error => 31, // Red
            log::Level::Warn => 93,  // BrightYellow
            log::Level::Info => 34,  // Blue
            log::Level::Debug => 32, // Green
            log::Level::Trace => 90, // BrightBlack
        };
        println!(
            "\x1b[{}m[{}] {}\x1b[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some(x) => match log::LevelFilter::from_str(x) {
            Ok(filter) => filter,
            Err(_) => log::LevelFilter::Off,
        },
        _ => log::LevelFilter::Off,
    });
}
