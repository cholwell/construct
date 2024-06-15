use console::Term;
use std::io::Result;

pub trait View {
    fn title(&self) -> String;
    fn content(&self, terminal: &Term, construct: &Construct) -> Result<()>;
}

pub struct Construct {
    terminal: Term,
}

impl Construct {
    pub fn new() -> Self {
        Self {
            terminal: Term::stdout(),
        }
    }

    pub fn navigate(&self, view: impl View) -> Result<()> {
        let (_, lines) = self.terminal.size();
        self.terminal.clear_last_lines(lines.into())?;
        self.terminal.write_line(&view.title())?;
        view.content(&self.terminal, &self)
    }
}
