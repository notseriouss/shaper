mod tree;
mod tui;

pub use self::{
    tree::{TreeNode, build_tree},
    tui::{Tui,},
};

use std::sync::{Arc,};

pub fn tui_run<'cfg, 'res>(
    output: Arc<dyn crate::ports::IOutput>,
    fs:     Arc<dyn crate::ports::IFileSystem>,
    config: &'cfg crate::domain::Config,
) -> crate::Result<Box<dyn crate::ports::ICommand + 'res>>
where
    'cfg: 'res,
{
    let root: self::TreeNode<'res> = self::build_tree(config.get_templates());
    let mut tui: self::Tui = self::Tui::init(root)?;

    match tui.run()? {
        Some(c) => Ok(Box::new(crate::application::ApplyCommand::<'res>::new(output, fs, config.get_options().clone(), vec![c]))),
        None    => Ok(Box::new(crate::application::CanceledCommand::new())),
    }
}





