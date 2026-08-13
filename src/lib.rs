pub mod domain;
pub mod ports;
pub mod adapters;
pub mod application;
pub mod error;

pub use error::{Error, Result,};

pub fn get_log_level() -> log::LevelFilter {
    match std::env::var("RUST_LOG")
        .ok()
        .as_deref()
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("error") => log::LevelFilter::Error,
        Some("warn")  => log::LevelFilter::Warn,
        Some("info")  => log::LevelFilter::Info,
        Some("debug") => log::LevelFilter::Debug,
        Some("trace") => log::LevelFilter::Trace,
        _             => log::LevelFilter::Info,
    }
}

pub fn get_config_path() -> crate::Result<std::path::PathBuf> {
    match std::env::var("SHAPER_CONFIG_PATH") {
        Ok(s)  => Ok(s.into()),
        Err(_) => match std::env::var("HOME") {
            Ok(s)  => Ok(std::path::PathBuf::from(s).join(".config/shaper/config.toml")),
            Err(_) => Err(crate::Error::Config(format!("Error: path to config.toml was not found, consider setting SHAPER_CONFIG_PATH"))),
        }
    }
}
