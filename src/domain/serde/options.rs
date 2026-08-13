#[derive(Debug, serde::Deserialize)]
pub struct FileOptions {
    pub templates_dir: std::path::PathBuf,

    #[serde(default)]
    pub dryrun:    Option<bool>,

    #[serde(default)]
    pub overwrite: Option<bool>,
}
