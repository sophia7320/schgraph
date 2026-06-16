use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::{
        Paragraph, Wrap,
        canvas::{self, Canvas},
    },
};

use crate::{
    App,
    base::ui::shortest_path::{circle_positions, parse_to_list},
};

pub(super) fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let [top, body] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)])
        .spacing(1)
        .areas(area);

    render_answer(app, top, frame);
    render_canvas(app, body, frame);
}

fn render_answer(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).spacing(3);

    let [path_area, cost_area] = area.layout(&layout);

    let path_text = app
        .gra
        .tsp_path()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" -> ");

    frame.render_widget(
        Paragraph::new(path_text)
            .style(Color::Red)
            .wrap(Wrap { trim: true }),
        path_area,
    );

    let cost_text = match app.gra.tsp_cost() {
        Some(cost) => cost.to_string(),
        None => "∞".to_string(),
    };
    let cost_text = "cost:\n".to_string() + cost_text.as_str();

    frame.render_widget(
        Paragraph::new(cost_text).alignment(Alignment::Left),
        cost_area,
    );
}

fn render_canvas(app: &mut App, area: Rect, frame: &mut Frame) {
    let n = app.gra.cnt;
    let pos = circle_positions(n, 9_f64);

    let canvas = Canvas::default()
        .x_bounds([-10.0, 10.0])
        .y_bounds([-10.0, 10.0])
        .paint(|ctx| {
            app.gra
                .linking
                .iter()
                .enumerate()
                .for_each(|(src, neibers)| {
                    neibers.iter().for_each(|(dst, _)| {
                        if src < *dst {
                            ctx.draw(&canvas::Line {
                                x1: pos[src].0,
                                y1: pos[src].1,
                                x2: pos[*dst].0,
                                y2: pos[*dst].1,
                                color: Color::Cyan,
                            });
                        }
                    })
                });


            let hl_edges = parse_to_list(app.gra.tsp_path());

            hl_edges.iter().for_each(|(src, dst)| {
                ctx.draw(&canvas::Line {
                    x1: pos[*src].0,
                    y1: pos[*src].1,
                    x2: pos[*dst].0,
                    y2: pos[*dst].1,
                    color: Color::Green,
                });
            });
            pos.iter().enumerate().for_each(|(id, &(x, y))| {
                ctx.draw(&canvas::Circle {
                    x,
                    y,
                    radius: 0.4,
                    color: Color::LightYellow,
                });
                ctx.print(x, y, Line::from(format!("{}", id)).style(Color::Red));
            });
        });

    frame.render_widget(canvas, area);
}
