use std::{io, panic};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

use crate::{
    App,
    base::{events::EventHandler, ui},
};

pub type CrosstermTerminal = ratatui::Terminal<CrosstermBackend<io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,

    pub events: EventHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();

        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor();
        self.terminal.clear();

        Ok(())
    }

    pub fn draw(&mut self, app: &App) -> Result<()> {
        self.terminal.draw(|f| ui::render(app, f))?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}
