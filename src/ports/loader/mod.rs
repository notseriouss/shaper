pub trait IConfigLoader: Send + Sync {
    fn load(&self, path: std::path::PathBuf) -> crate::Result<crate::domain::Config>;
    fn validate(&self, config: &crate::domain::Config) -> crate::Result<()>;
}
