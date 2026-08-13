#[derive(Debug)]
pub struct Template {
    folder:      String,
    name:        String,
    description: String,
    groups:      Vec<String>,
    source_path: std::path::PathBuf,
}

impl Template {
    #[inline]
    pub fn new(
        folder:      String,
        name:        String,
        description: String,
        groups:      Vec<String>,
        source_path: std::path::PathBuf,
    ) -> Self {
        Self { folder, name, description, groups, source_path, }
    }

    #[inline]
    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_description(&self) -> &str {
        &self.description
    }

    #[inline]
    pub fn get_groups(&self) -> &[String] {
        &self.groups
    }

    #[inline]
    pub fn get_source_path(&self) -> &std::path::Path {
        self.source_path.as_path()
    }
}
