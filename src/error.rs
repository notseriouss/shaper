use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("filesystem copy error: {0}")]
    FsExtra(#[from] fs_extra::error::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Log error: {0}")]
    Log(String),

    #[error("Mutex lock error: {0}")]
    MutexPoisoned(String),

    #[error("Command error: {0}")]
    Command(String),

    #[error("FileSystem error: {0}")]
    Fs(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("CLI error: {0}")]
    Cli(String),

    #[error("TUI error: {0}")]
    Tui(String),

}
