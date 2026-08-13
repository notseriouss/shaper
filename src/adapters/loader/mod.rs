use std::sync::{Arc,};

pub struct TomlConfigLoader {
    fs: Arc<dyn crate::ports::IFileSystem>,
}

impl TomlConfigLoader {
    #[inline]
    pub fn new(fs: Arc<dyn crate::ports::IFileSystem>) -> Self {
        Self { fs, }
    }
}

impl crate::ports::IConfigLoader for TomlConfigLoader {
    fn load(&self, path: std::path::PathBuf) -> crate::Result<crate::domain::Config> {
        let content: String = self.fs.read_to_string(&path)?;
        let file_config: crate::domain::FileConfig = toml::from_str(&content)?;

        let templates: Vec<crate::domain::Template> = file_config.templates
            .into_iter()
            .map(|template| {
                crate::domain::Template::new(
                    template.folder.clone(),
                    template.name.unwrap_or(template.folder.clone()),
                    template.description.unwrap_or("No description".to_string()),
                    template.groups,
                    file_config.options.templates_dir.join(template.folder),
                )
            })
            .collect();
        let options: crate::domain::Options = crate::domain::Options::from(file_config.options);

        let cfg: crate::domain::Config = crate::domain::Config::new(options, templates)?;
        self.validate(&cfg)?;

        Ok(cfg)
    }

    fn validate(&self, config: &crate::domain::Config) -> crate::Result<()> {
        self.fs.validate_path(config.get_options().get_templates_path())?;

        for template in config.get_templates() {
            self.fs.validate_path(template.get_source_path())?;
        }

        Ok(())
    }
}
