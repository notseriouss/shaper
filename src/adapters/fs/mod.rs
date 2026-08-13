pub struct FileSystem;
impl crate::ports::IFileSystem for FileSystem {
    fn validate_path(&self, path: &std::path::Path) -> crate::Result<()> {
        if !path.exists() && true { // todo
            return Err(crate::Error::Fs(format!("Invalid path: {}", path.to_string_lossy())));
        }

        Ok(())
    }

    #[inline]
    fn read_to_string(&self, path: &std::path::Path)  -> crate::Result<String> {
        match std::fs::read_to_string(path) {
            Ok(c)  => Ok(c),
            Err(e) => Err(crate::Error::Io(e)),
        }
    }

    #[inline]
    fn get_current_dir(&self) -> crate::Result<std::path::PathBuf> {
        match std::env::current_dir() {
            Ok(p)  => Ok(p),
            Err(e) => Err(crate::Error::Io(e)),
        }
    }

    fn copy_directory(&self, source: &std::path::Path, target: &std::path::Path, overwrite: bool) -> crate::Result<()> {
        let copy_options: fs_extra::dir::CopyOptions = fs_extra::dir::CopyOptions::new()
            .overwrite(overwrite)
            .content_only(true);

        match fs_extra::dir::copy(source, target, &copy_options) {
            Ok(_)  => Ok(()),
            Err(e) => return Err(crate::Error::FsExtra(e)),
        }
    }
}
