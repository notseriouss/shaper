#[derive(Debug, serde::Deserialize)]
pub struct FileTemplate {
    pub folder: String,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub groups: Vec<String>,
}
