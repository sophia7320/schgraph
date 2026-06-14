#![allow(unused_variables)]
#![allow(dead_code)]
use std::{collections::HashMap, io};

use color_eyre::eyre::Result;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::base::{
    events::{Event, EventHandler},
    filescanner::FileScanner,
    graph::Graph,
    tui::Tui,
    ui::View,
    update::{self, Action},
};

#[derive(Debug, Default)]
pub struct App {
    should_exit: bool,
    pub(super) view: View,

    pub(super) places: Vec<Place>,
    pub(super) gra: Graph,
}

#[derive(Debug)]
pub struct Place {
    id: usize,
    name: String,
    desc: String,
}

impl Place {
    pub fn new(id: usize, name: String, desc: String) -> Place {
        Place { id, name, desc }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn from_file(filepath: &str) -> Result<Self> {
        let mut app = App::new();
        let scanner = FileScanner::from_file_path(filepath);
        let mut input = scanner.iter();

        app.places = input.get_places();
        let mut mapper = HashMap::with_capacity(app.places.len());

        app.places.iter().enumerate().for_each(|(id, place)| {
            mapper.insert(&place.name, id);
        });

        let edges: Vec<(usize, usize, u64)> = input
            .get_raw_edges()
            .iter()
            .map(|(u, v, w)| (*mapper.get(u).unwrap(), *mapper.get(v).unwrap(), *w))
            .collect();

        app.gra = Graph::new(app.places.len(), edges);

        Ok(app)
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
