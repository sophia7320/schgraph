use std::f64::consts::PI;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, List, Paragraph, Tabs,
        canvas::{self, Canvas},
    },
};

use crate::{
    App,
    base::app::{Place, SP_InputFocus},
};

pub fn render(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Length(4),
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
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]);

    let [desc, line] = area.layout(&layout);

    //render desc
    let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(1);
    let [desc_left, desc_right] = desc.layout(&layout);
    frame.render_widget(Paragraph::new("src").centered().bold(), desc_left);
    frame.render_widget(Paragraph::new("dst").centered().bold(), desc_right);

    let [input_left, input_right] = line.layout(&layout);

    let (focus_color, unfocus_color) = (Color::Green, Color::White);

    let (left_color, right_color) = match app.sp_input_focus {
        SP_InputFocus::Left => (focus_color, unfocus_color),
        SP_InputFocus::Right => (unfocus_color, focus_color),
    };

    frame.render_widget(
        Paragraph::new(app.sp_input_left.as_str().white())
            .centered()
            .block(Block::bordered().style(left_color)),
        input_left,
    );
    frame.render_widget(
        Paragraph::new(app.sp_input_right.as_str().white())
            .centered()
            .block(Block::bordered().style(right_color)),
        input_right,
    );
}

fn render_answer(app: &mut App, area: Rect, frame: &mut Frame) {
    let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).spacing(1);

    let [path_area, cost_area] = area.layout(&layout);

    let path_text = app
        .sp_path
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" -> ");

    frame.render_widget(Paragraph::new(path_text).style(Color::Red), path_area);

    let cost_text = match app.sp_cost {
        Some(cost) => cost.to_string(),
        None => "∞".to_string(),
    };
    let cost_text = "cost:\n".to_string() + cost_text.as_str();

    frame.render_widget(
        Paragraph::new(cost_text).alignment(Alignment::Left),
        cost_area,
    );
}

fn render_body(app: &mut App, area: Rect, frame: &mut Frame) {
    render_tags(app, area + Offset::new(1, -1), frame);

    match app.sp_select {
        0 => render_idex_map(app, area, frame),
        1 => render_canvas(app, area, frame),
        2 => render_place_details(app, area, frame),
        _ => {}
    }
}

fn render_tags(app: &mut App, area: Rect, frame: &mut Frame) {
    let tabs = Tabs::new(["id_mapping", "visalgraph", "place_desc"])
        .style(Color::White)
        .highlight_style(Style::default().red().on_black().bold())
        .select(app.sp_select)
        .divider(symbols::DOT)
        .padding(" ", " ");

    frame.render_widget(tabs, area);
}

pub(super) fn circle_positions(n: usize, r: f64) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            let angle = 2.0 * PI * i as f64 / n as f64;
            (r * angle.cos(), r * angle.sin())
        })
        .collect()
}

pub(super) fn parse_to_list(path: &[usize]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let n = path.len();
    if n == 0 {
        return res;
    }

    for i in 0..(n - 1) {
        res.push((path[i], path[i + 1]));
    }
    res
}

fn render_idex_map(app: &mut App, area: Rect, frame: &mut Frame) {
    let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)])
        .spacing(1)
        .areas(area);

    let max_rows = left.height as usize;
    let mid = app.places.len().min(max_rows);

    let places = &app.places;
    let left_its: Vec<Line> = places[..mid]
        .iter()
        .map(|p| Line::from(format!("  {}. {}", p.id, p.name)))
        .collect();
    let right_its: Vec<Line> = places[mid..]
        .iter()
        .map(|p| Line::from(format!("  {}. {}", p.id, p.name)))
        .collect();

    frame.render_widget(List::new(left_its), left);
    frame.render_widget(List::new(right_its), right);
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

            let hl_edges = parse_to_list(&app.sp_path);

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

fn render_place_details(app: &mut App, area: Rect, frame: &mut Frame) {
    let places: Vec<&Place> = app.sp_path.iter().map(|i| &app.places[*i]).collect();

    let descs = places
        .iter()
        .map(|p| format!("id:{} {}\n{}", p.id, p.name, p.desc))
        .collect::<Vec<String>>()
        .join("\n\n");

    frame.render_widget(Paragraph::new(descs), area);
}
