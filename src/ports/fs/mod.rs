pub trait IFileSystem: Send + Sync {
    fn validate_path(&self, path: &std::path::Path) -> crate::Result<()>;  
    fn read_to_string(&self, path: &std::path::Path) -> crate::Result<String>;
    fn get_current_dir(&self) -> crate::Result<std::path::PathBuf>;
    fn copy_directory(&self, source: &std::path::Path, target: &std::path::Path, overwrite: bool) -> crate::Result<()>;
}
