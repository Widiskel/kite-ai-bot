use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
};

pub struct CustomLogger;

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => "ERROR",
                Level::Warn => "WARN",
                Level::Info => "INFO",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            };

            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

            let _ = create_dir_all("log");

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("log/app.log")
                .unwrap();

            let _ = writeln!(
                &mut file,
                "[{}] [{}] [{}] {}",
                timestamp,
                level,
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    let _ = File::create("log/app.log");

    log::set_boxed_logger(Box::new(CustomLogger))?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}
