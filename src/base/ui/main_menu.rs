use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Stylize},
    text::{Line, Span},
    widgets::List,
};

use crate::App;

pub(super) fn render(app: &mut App, frame: &mut Frame) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Percentage(10),
        Constraint::Fill(1),
        Constraint::Length(1),
    ];

    let layout = Layout::vertical(constraints).spacing(1);

    let [top, top_desc, _, body, bottom] = frame.area().layout(&layout);

    let title = Line::from_iter([Span::from("-- SchoolGraph -- ").bold()]).centered();

    let desc = Line::from("(Press 'q' to quit and arrow keys to navigate)").centered();

    frame.render_widget(title, top);
    frame.render_widget(desc, top_desc);

    render_options(app, body, frame);

    frame.render_widget(Line::from("this is bottom"), bottom);
}

fn render_options(app: &mut App, area: Rect, frame: &mut Frame) {
    let items = [
        "show places desc",
        "show Adjacency matrix",
        "show Adjacency list",
        "find the shortest between two places",
        "show the tsp(visit all places and return to beginning) circle",
    ];

    let list = List::new(items)
        .style(Color::Yellow)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol(">");

    let list_state = &mut app.main_menu_statu;

    let layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(85),
        Constraint::Fill(1),
    ]);

    let [_, area, _] = area.layout(&layout);

    frame.render_stateful_widget(list, area, list_state);
}
