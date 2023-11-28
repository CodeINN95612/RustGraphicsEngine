use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record};

pub static LOGGER: Logger = Logger;

pub struct Logger;

impl Logger {
    pub fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let date = Local::now();
            let formatted_date = date.format("%Y/%m/%d %H:%M:%S").to_string();
            println!(
                "\"{}\" {} {} [{}] {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                formatted_date,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
