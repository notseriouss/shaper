pub trait IOutput: Send + Sync {
    fn println(&self, args: std::fmt::Arguments<'_>) -> crate::Result<()>;
    fn print(&self, args: std::fmt::Arguments<'_>) -> crate::Result<()>;
}
