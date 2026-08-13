use crossterm::style::Stylize;

pub enum ItemRef<'cfg> {
    ParentDir,
    Child(&'cfg super::TreeNode<'cfg>),
}

pub struct Tui<'cfg> {
    running:   bool,
    root:      super::TreeNode<'cfg>,
    sel_index: usize,
    scrolloff: usize,
    current_path: Vec<usize>,
}

impl<'cfg, 'res> Tui<'cfg>
where
    'cfg: 'res
{
    pub fn init(root: super::TreeNode<'cfg>) -> crate::Result<self::Tui<'res>> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;

        Ok(self::Tui::<'res> {
            running: false,
            root,
            sel_index: 0usize,
            scrolloff: 0usize,
            current_path: Vec::new(),
        })
    }

    pub fn run(&mut self) -> crate::Result<Option<&'res crate::domain::Template>> {
        self.running = true;

        let chosen: Option<&'res crate::domain::Template> = loop {
            if !self.running { break None; }
            self.render()?;

            match self.handle_key()? {
                Some(c) => break Some(c),
                None    => continue,
            }
        };

        Ok(chosen)
    }

    pub(self) fn handle_key(&mut self) -> crate::Result<Option<&'res crate::domain::Template>> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key) => match key.code {
                crossterm::event::KeyCode::Esc => {
                    self.restore()?;
                    return Ok(None);
                }
                crossterm::event::KeyCode::Enter => {
                    let items = self.items();
                    if items.is_empty() {
                        return Ok(None);
                    }
                    let is_root = self.current_path.is_empty();

                    if !is_root && self.sel_index == 0 {
                        self.current_path.pop();
                        self.sel_index = 0;
                        self.scrolloff = 0;
                    } else {
                        let child_idx = self.sel_index - if is_root { 0 } else { 1 };
                        let current = self.current_node();
                        let child = &current.get_children()[child_idx];

                        if let Some(template) = child.get_template() {
                            self.restore()?;
                            return Ok(Some(template));
                        } else {
                            self.current_path.push(child_idx);
                            self.sel_index = 0;
                            self.scrolloff = 0;
                        }
                    }
                }
                crossterm::event::KeyCode::Up => {
                    let items = self.items();
                    if !items.is_empty() && self.sel_index > 0 {
                        self.sel_index -= 1;
                        if self.sel_index < self.scrolloff {
                            self.scrolloff = self.sel_index;
                        }
                    }
                }
                crossterm::event::KeyCode::Down => {
                    let items = self.items();
                    if !items.is_empty() && self.sel_index + 1 < items.len() {
                        self.sel_index += 1;
                        let (_, rows) = crossterm::terminal::size()?;
                        let visible_rows = rows as usize;
                        if self.sel_index >= self.scrolloff + visible_rows {
                            self.scrolloff = self.sel_index - visible_rows + 1;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(None)
    }

    pub(self) fn render(&mut self) -> crate::Result<()> {
        let mut stdout = std::io::stdout();

        let items = self.items();
        let (cols, rows) = crossterm::terminal::size()?;
        let visible_rows = rows as usize;

        crossterm::queue!(&mut stdout, crossterm::cursor::MoveTo(0, 0))?;

        for line in 0..visible_rows {
            let item_idx = self.scrolloff + line;
            if item_idx >= items.len() {
                crossterm::queue!(
                    &mut stdout,
                    crossterm::style::Print(format!("{:<width$}", "", width = cols as usize)),
                    crossterm::cursor::MoveToNextLine(1),
                    crossterm::cursor::MoveToColumn(0)
                )?;
                continue;
            }

            let is_selected = item_idx == self.sel_index;
            let item = &items[item_idx];
            let (name, is_folder) = match item {
                ItemRef::ParentDir => ("..", true),
                ItemRef::Child(child) => (&child.get_name()[..], child.get_template().is_none()),
            };

            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, name);
            let padded = format!("{:<width$}", text, width = cols as usize);
            let color = if is_folder { crossterm::style::Color::Blue } else { crossterm::style::Color::White };

            let styled = if is_selected {
                crossterm::style::style(padded)
                    .on(crossterm::style::Color::AnsiValue(237))
                    .with(color)
            } else {
                crossterm::style::style(padded).with(color)
            };

            crossterm::queue!(
                &mut stdout,
                crossterm::style::PrintStyledContent(styled),
                crossterm::cursor::MoveToNextLine(1),
                crossterm::cursor::MoveToColumn(0)
            )?;
        }

        std::io::Write::flush(&mut stdout)?;

        Ok(())
    }

    pub(self) fn items(&'res self) -> Vec<ItemRef<'res>> {
        let node = self.current_node();
        let mut items = Vec::new();
        if !self.current_path.is_empty() {
            items.push(ItemRef::ParentDir);
        }
        for child in node.get_children().iter() {
            items.push(ItemRef::Child(child));
        }
        items
    }

    pub(self) fn current_node(&'res self) -> &'res super::TreeNode<'cfg> {
        let mut node = &self.root;
        for &idx in &self.current_path {
            node = &node.get_children()[idx];
        }
        node
    }

    pub(self) fn restore(&mut self) -> crate::Result<()> {
        if self.running {
            self.running = false;
            crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }

        Ok(())
    }
}
impl<'cfg> Drop for Tui<'cfg> {
    fn drop(&mut self) {
        match self.restore() {
            Ok(()) => {},
            Err(e) => log::error!("{}", e),
        }
    }
}

