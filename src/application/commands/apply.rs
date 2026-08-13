use std::sync::{Arc,};

pub struct ApplyCommand<'cfg> {
    output:   Arc<dyn crate::ports::IOutput>,
    fs:       Arc<dyn crate::ports::IFileSystem>,
    options:  crate::domain::Options,
    selected: Vec<&'cfg crate::domain::Template>,
}

impl<'cfg> ApplyCommand<'cfg> {
    #[inline]
    pub(crate) fn new(
        output:   Arc<dyn crate::ports::IOutput>,
        fs:       Arc<dyn crate::ports::IFileSystem>,
        options:  crate::domain::Options,
        selected: Vec<&'cfg crate::domain::Template>,
    ) -> Self {
        Self { output, fs, options, selected, }
    }
}

impl<'cfg> crate::ports::ICommand for ApplyCommand<'cfg> {
    fn perform(&mut self) -> crate::Result<()> {
        let target: std::path::PathBuf = self.fs.get_current_dir()?;
        self.fs.validate_path(&target)?;

        if self.options.get_dryrun() {
            self.output.println(format_args!("The following templates would have been applied:\n"))?;
            crate::application::ListCommand::<'cfg>::new(self.output.clone(), self.selected.clone()).perform()?;
            return Ok(());
        }

        for template in &self.selected {
            self.fs.copy_directory(template.get_source_path(), &target, self.options.get_overwrite())?;
        }

        log::info!("OK");
        Ok(())
    }
}
