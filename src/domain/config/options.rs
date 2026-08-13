#[derive(Debug, Clone)]
pub struct Options {
    templates_dir: std::path::PathBuf,
    dryrun:        bool,
    overwrite:     bool,
}

impl Options {
    #[inline]
    pub fn new(
        templates_dir: std::path::PathBuf,
        dryrun:        bool,
        overwrite:     bool,
    ) -> Self {
        Self { templates_dir, dryrun, overwrite, }
    }

    #[inline]
    pub fn get_templates_path(&self) -> &std::path::Path {
        self.templates_dir.as_path()
    }

    #[inline]
    pub fn get_dryrun(&self) -> bool {
        self.dryrun
    }

    #[inline]
    pub fn get_overwrite(&self) -> bool {
        self.overwrite
    }
}

impl From<crate::domain::serde::FileOptions> for Options {
    #[inline]
    fn from(value: crate::domain::serde::FileOptions) -> Options {
        Options {
            templates_dir: value.templates_dir,
            dryrun:        value.dryrun.unwrap_or(false),
            overwrite:     value.overwrite.unwrap_or(false),
        }
    }
}
