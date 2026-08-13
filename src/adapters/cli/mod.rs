pub mod args;

pub use self::{
    args::{CliArgs, CliCommand,},
};

use std::sync::{Arc,};

pub fn cli_run<'cfg, 'res>(
    cmd:    crate::adapters::CliCommand,
    output: Arc<dyn crate::ports::IOutput>,
    fs:     Arc<dyn crate::ports::IFileSystem>,
    config: &'cfg crate::domain::Config,
) -> crate::Result<Box<dyn crate::ports::ICommand + 'res>>
where
    'cfg: 'res,
{
    match cmd {
        crate::adapters::CliCommand::Apply { apply_selected, apply_options } => {
            if apply_selected.is_empty() {
                return Err(crate::Error::Cli(format!("No templates were selected")));
            }

            let selected: Vec<&'res crate::domain::Template> = self::select(apply_selected, config.get_templates())?;
            let merged_options: crate::domain::Options = crate::domain::Options::new(
                config.get_options().get_templates_path().to_path_buf(),
                apply_options.dryrun.unwrap_or(config.get_options().get_dryrun()),
                apply_options.overwrite.unwrap_or(config.get_options().get_overwrite()),
            );

            Ok(Box::new(crate::application::ApplyCommand::<'res>::new(output, fs, merged_options, selected)))
        },

        crate::adapters::CliCommand::List => {
            Ok(Box::new(crate::application::ListCommand::<'res>::new(output, config.get_templates().iter().collect())))
        },
    }
}

pub(self) fn select<'cfg, 'res>(
    apply_selected: Vec<String>,
    templates:      &'cfg [crate::domain::Template]
) -> crate::Result<Vec<&'res crate::domain::Template>>
where
    'cfg: 'res,
{
    let mut selected: Vec<&'res crate::domain::Template> = Vec::with_capacity(apply_selected.len());
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for selection in apply_selected {
        let template: &'res crate::domain::Template = match templates.iter().find(|t| t.get_folder() == selection) {
            Some(te) => te,
            None     => return Err(crate::Error::Cli(format!("Entry \"{}\" not found", &selection))),
        };

        if seen.insert(selection) {
            selected.push(template);
        }
    }

    Ok(selected)
}

