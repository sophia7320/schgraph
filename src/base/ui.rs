use color_eyre::eyre::Result;
use ratatui::Frame;

use crate::App;

#[derive(Debug)]
pub(super) enum View {
    MainMenu,
    AdjacencyMatrix,
    AdjacencyList,
    VisualGraph,
    ShortestPath,
    Tsp,
}

impl Default for View {
    fn default() -> Self {
        View::MainMenu
    }
}

pub fn render(app: &App, frame: &mut Frame) {
    match app.view {
        View::MainMenu => todo!(),
        _ => {}
    }
}
