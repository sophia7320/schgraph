use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::{App, base::app::Place};

pub(super) fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let places: &Vec<Place> = &app.places;

    let descs = places
        .iter()
        .map(|p| format!("id:{} {}\n{}", p.id, p.name, p.desc))
        .collect::<Vec<String>>()
        .join("\n\n");

    frame.render_widget(Paragraph::new(descs), area);
}
