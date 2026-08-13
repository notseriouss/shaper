#[derive(clap::Args, Debug)]
pub struct ApplyOptions {
    //#[arg(long = "target")]
    //pub(crate) target: Option<std::path::PathBuf>,

    #[arg(long = "dryrun", action = clap::ArgAction::SetTrue)]
    pub dryrun: Option<bool>,

    #[arg(long = "overwrite", action = clap::ArgAction::SetTrue)]
    pub overwrite: Option<bool>,
}

#[derive(clap::Subcommand, Debug)]
pub enum CliCommand {
    Apply {
        #[arg(value_name = "templates")]
        apply_selected: Vec<String>,

        #[command(flatten)]
        apply_options: ApplyOptions,
    },

    List,
}

#[derive(clap::Parser, Debug)]
#[command(name = "Shaper", about = "Template manager")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}
