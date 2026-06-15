mod main_menu;

use crate::App;
use ratatui::Frame;

#[derive(Debug, Default)]
pub(super) enum View {
    #[default]
    MainMenu,
    AdjacencyMatrix,
    AdjacencyList,
    VisualGraph,
    ShortestPath,
    Tsp,
}

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.view {
        View::MainMenu => main_menu::render(app, frame),
        _ => todo!(),
    }
}
