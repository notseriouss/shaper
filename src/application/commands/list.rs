use std::sync::{Arc,};

pub struct ListCommand<'cfg> {
    output:    Arc<dyn crate::ports::IOutput>,
    templates: Vec<&'cfg crate::domain::Template>,
}

impl<'cfg> ListCommand<'cfg> {
    #[inline]
    pub fn new(
        output:    Arc<dyn crate::ports::IOutput>,
        templates: Vec<&'cfg crate::domain::Template>,
    ) -> Self {
        Self { output, templates, }
    }
}

impl<'cfg> crate::ports::ICommand for ListCommand<'cfg> {
    fn perform(&mut self) -> crate::Result<()> {
        for (index, template) in self.templates.iter().enumerate() {
            self.output.println(format_args!("{:<2} {:<20} {:<30} {}", //todo
                index+1,
                template.get_name(),
                template.get_folder(),
                template.get_description()
            ))?
        }
        Ok(())
    }
}
