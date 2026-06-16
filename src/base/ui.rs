mod adj_matri;
mod graph_canvas;
mod main_menu;
mod shortest_path;
mod tsp;

use crate::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Span},
};

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
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, content] = frame.area().layout(&layout);

    let title = Line::from_iter([Span::from("-- SchoolGraph -- ").bold()]).centered();

    frame.render_widget(title, top);

    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(85),
        Constraint::Fill(1),
    ]);

    let [_, area, _] = content.layout(&layout);

    match app.view {
        View::MainMenu => main_menu::render(app, area, frame),
        View::AdjacencyMatrix => adj_matri::render(app, area, frame),
        View::ShortestPath => shortest_path::render(app, area, frame),
        View::Tsp => tsp::render(app, area, frame),
        _ => todo!(),
    }
}
