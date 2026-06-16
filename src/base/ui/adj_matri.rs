use std::iter::{self, once};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::{Row, Table, TableState},
};

use crate::App;

pub(super) fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let title = Line::from("----------------AdjAmatriOfGraph--------------------".to_string())
        .style(Color::Green)
        .centered();

    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);

    let [top, body] = area.layout(&layout);

    frame.render_widget(title, top);
    render_matri(app, body, frame);
}

fn render_matri(app: &mut App, area: Rect, frame: &mut Frame) {
    let graph = &app.gra;
    let n = graph.cnt;

    let mat = iter::once(Row::new(
        once(String::default()).chain((0..n).map(|x| x.to_string())),
    ))
    .chain((0..n).map(|row| {
        Row::new(once(row.to_string()).chain((0..n).map(|col| graph.matri[row][col].to_string())))
    }));

    let widths = (0..=n).map(|_| Constraint::Length(3));

    let mat_table = Table::new(mat, widths).style(Color::Blue).column_spacing(1);

    let mat_state = &mut TableState::default();

    frame.render_widget(mat_table, area);
    // frame.render_stateful_widget(mat_table, area, mat_state);
}
