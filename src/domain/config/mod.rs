pub mod template;
pub mod options;

pub use self::{
    template::Template,
    options::Options,
};

#[derive(Debug)]
pub struct Config {
    options:   Options,
    templates: Vec<Template>,
}

impl Config {
    pub fn new(options: Options, templates: Vec<Template>) -> crate::Result<Self> {
        let cfg: Self = Self { options, templates, };

        if cfg.templates.is_empty() {
            return Err(crate::Error::Config(format!("No [[template]] defined")));
        }
        
        let mut seen = std::collections::HashSet::new();
        for t in &cfg.templates {
            if !seen.insert(t.get_folder()) {
                return Err(crate::Error::Config(format!("Duplicated template: {}", t.get_folder())));
            }
        }

        Ok(cfg)
    }

    #[inline]
    pub fn get_options(&self) -> &Options {
        &self.options
    }

    #[inline]
    pub fn get_templates(&self) -> &[Template] {
        &self.templates
    }
}
