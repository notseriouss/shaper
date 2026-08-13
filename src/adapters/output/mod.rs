pub struct ConsoleOutput {
    stdout: std::sync::Mutex<std::io::Stdout>,
}

impl ConsoleOutput {
    #[inline]
    pub fn new() -> Self {
        Self { stdout: std::sync::Mutex::new(std::io::stdout()) }
    }
}

impl crate::ports::IOutput for ConsoleOutput {
    fn println(&self, args: std::fmt::Arguments<'_>) -> crate::Result<()> {
        let mut stdout = self.stdout
            .lock()
            .map_err(|e| crate::Error::MutexPoisoned(e.to_string()))?;

        std::io::Write::write_fmt(&mut *stdout, format_args!("{}\n", args))?;
        std::io::Write::flush(&mut *stdout)?;
        Ok(())
    }

    fn print(&self, args: std::fmt::Arguments<'_>) -> crate::Result<()> {
        let mut stdout = self.stdout
            .lock()
            .map_err(|e| crate::Error::MutexPoisoned(e.to_string()))?;

        std::io::Write::write_fmt(&mut *stdout, args)?;
        std::io::Write::flush(&mut *stdout)?;
        Ok(())
    }
}
