use std::io;

use color_eyre::eyre::Result;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::base::{
    events::{Event, EventHandler},
    graph::Graph,
    tui::Tui,
    ui::View,
    update::{self, Action},
};

#[derive(Debug, Default)]
pub struct App {
    should_exit: bool,

    pub(super) view: View,
    pub(super) gra: Graph,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn run(mut self) -> Result<()> {
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(250);

        let mut tui = Tui::new(terminal, events);

        tui.enter()?;

        while !self.should_exit {
            tui.draw(&self)?;

            let action = match tui.events.next()? {
                Event::Tick => Action::Noop,
                Event::Key(key) => update::key_event_handle(key, &self.view),
                _ => Action::Noop,
            };

            self.update(action);
        }

        Ok(())
    }

    fn update(&mut self, action: Action) {}
}
