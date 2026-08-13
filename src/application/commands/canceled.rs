pub struct CanceledCommand;
impl CanceledCommand {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::ports::ICommand for CanceledCommand {
    fn perform(&mut self) -> crate::Result<()> {
        log::info!("Canceled");
        Ok(())
    }
}
