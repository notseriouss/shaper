use std::sync::{Arc,};

pub struct Logger {
    output: Arc<dyn crate::ports::IOutput>,
}

impl Logger {
    #[inline]
    pub fn new(output: Arc<dyn crate::ports::IOutput>) -> Self {
        Self { output, }
    }
}

impl log::Log for Logger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level_style: &str = match record.level() {
                log::Level::Error => "\x1b[31m",
                log::Level::Info  => "\x1b[32m",
                log::Level::Warn  => "\x1b[33m",
                log::Level::Debug => "\x1b[34m",
                log::Level::Trace => "\x1b[35m",
            }; // todo

            let _: _ = self.output.println(format_args!("{}{:<5}\x1b[0m > {}",level_style,record.level(),record.args()));
        }
    }

    fn flush(&self) {}
}
