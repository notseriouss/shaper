use shaper;
use std::sync::{LazyLock, Arc};

static FS: LazyLock<Arc<dyn shaper::ports::IFileSystem>> = LazyLock::new(|| {
    Arc::new(shaper::adapters::FileSystem)
});

static OUTPUT: LazyLock<Arc<dyn shaper::ports::IOutput>> = LazyLock::new(|| {
    Arc::new(shaper::adapters::ConsoleOutput::new())
});

static LOADER: LazyLock<Arc<dyn shaper::ports::IConfigLoader>> = LazyLock::new(|| {
    Arc::new(shaper::adapters::TomlConfigLoader::new(FS.clone()))
});

static LOGGER: LazyLock<shaper::adapters::Logger> = LazyLock::new(|| {
    shaper::adapters::Logger::new(OUTPUT.clone())
});

fn main() -> std::process::ExitCode {
    let args: shaper::adapters::CliArgs = clap::Parser::parse();

    match log::set_logger(&*LOGGER) {
        Ok(()) => log::set_max_level(shaper::get_log_level()),
        Err(e) => { eprintln!("error setting logger: {}", e); return std::process::ExitCode::FAILURE; },
    }

    let config_path: std::path::PathBuf = match shaper::get_config_path() {
        Ok(p)  => p,
        Err(e) => { log::error!("{}", e); return std::process::ExitCode::FAILURE; },
    };

    let config: shaper::domain::Config = match LOADER.load(config_path) {
        Ok(c)  => c,
        Err(e) => { log::error!("{}", e); return std::process::ExitCode::FAILURE; },
    };

    match run(args, config) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => { log::error!("{}", e); std::process::ExitCode::FAILURE },
    }
}

fn run(args: shaper::adapters::CliArgs, config: shaper::domain::Config) -> shaper::Result<()> {
    let mut command: Box<dyn shaper::ports::ICommand> = match args.command {
        Some(cmd) => shaper::adapters::cli_run(cmd, OUTPUT.clone(), FS.clone(), &config)?,
        None      => shaper::adapters::tui_run(     OUTPUT.clone(), FS.clone(), &config)?,
    };

    command.perform()
}
