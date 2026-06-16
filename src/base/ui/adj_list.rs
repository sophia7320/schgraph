use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::Paragraph,
};

use crate::App;

pub(super) fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let title = Line::from("----------------AdjListOfGraph--------------------".to_string())
        .style(Color::Green)
        .centered();

    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);

    let [top, body] = area.layout(&layout);

    frame.render_widget(title, top);
    render_adj_list(app, body, frame);
}

fn render_adj_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let list_context = app
        .gra
        .linking
        .iter()
        .enumerate()
        .map(|(id, neibers)| {
            let snip = neibers
                .iter()
                .map(|(v, w)| format!("({} ,{})", v, w))
                .collect::<Vec<String>>()
                .join(" , ");

            format!("{}: ", id) + snip.as_str()
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    frame.render_widget(Paragraph::new(list_context), area);
}
