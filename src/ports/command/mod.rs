pub trait ICommand: Send + Sync {
    fn perform(&mut self) -> crate::Result<()>;
}
