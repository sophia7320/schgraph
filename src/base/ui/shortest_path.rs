use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::App;

pub fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .spacing(1);

    let [input_line, answer, body] = area.layout(&layout);

    render_input_line(app, input_line, frame);
    render_answer(app, answer, frame);
    render_body(app, body, frame);
}

fn render_input_line(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);

    let [desc, line] = area.layout(&layout);

    //render desc
    let desc_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(1);

    let [desc_left, desc_right] = desc.layout(&desc_layout);

    todo!()
}

fn render_answer(app: &mut App, area: Rect, frame: &mut Frame) {
    todo!()
}

fn render_body(app: &mut App, area: Rect, frame: &mut Frame) {
    todo!()
}

fn render_idex_map(app: &mut App, area: Rect, frame: &mut Frame) {
    todo!()
}

fn render_canvas(app: &mut App, area: Rect, frame: &mut Frame) {
    todo!()
}

fn render_place_details(app: &mut App, area: Rect, frame: &mut Frame) {
    todo!()
}
