pub mod template;
pub mod options;

pub use self::{
    template::FileTemplate,
    options::FileOptions,
};

#[derive(Debug, serde::Deserialize)]
pub struct FileConfig {
    #[serde(flatten)]
    pub options: FileOptions,

    #[serde(default, rename = "template")]
    pub templates: Vec<FileTemplate>,
}
