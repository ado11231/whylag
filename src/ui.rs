use crate::{app::App, stats::Stats, status::Health};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Duration;

pub const ORANGE: Color = Color::Rgb(235, 163, 65);
/// Color for borders and the app name.
const ACCENT: Color = Color::Magenta;
/// Color for titles and labels.
const LABEL: Color = Color::Cyan;

/// Smallest screen that fits every panel.
const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 22;

/// A ping at or above this value fills the chart.
const CHART_MAX: f64 = 300.0;
/// Columns used by each bar.
const BAR_WIDTH: u16 = 2;
/// Empty columns between bars.
const BAR_GAP: u16 = 1;
/// Bar pieces from empty to full, in eighths of a row.
const BLOCKS: [&str; 9] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

pub fn latency_color(ping: Option<f64>) -> Color {
    match ping {
        None => Color::Reset,
        Some(value) if value < 50.0 => Color::Green,
        Some(value) if value <= 150.0 => ORANGE,
        _ => Color::Red,
    }
}

pub fn milliseconds(value: Option<f64>) -> String {
    value.map_or("n/a".into(), |value| format!("{value:.0} ms"))
}

/// Shows a duration as minutes and seconds.
pub fn clock(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

pub fn draw(frame: &mut Frame, app: &App, target: &str) {
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let message = format!("whylag\nResize to at least {MIN_WIDTH} x {MIN_HEIGHT}\nq quit");
        frame.render_widget(Paragraph::new(message).fg(LABEL), area);
        return;
    }
    let [top, middle, details_area, status_area, events_area, bottom] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(6),
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
    ])
    .areas(area);
    header(frame, top, app);
    let (stats, title) = if app.show_router {
        (&app.router, "Router latency".to_owned())
    } else {
        (&app.internet, format!("Internet latency  {target}"))
    };
    chart(frame, middle, stats, &title);
    details(frame, details_area, app, stats);
    status(frame, status_area, app);
    events(frame, events_area, app);
    footer(frame, bottom, app);
}

/// Network name and signal on the left, gateway on the right.
fn header(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::new().fg(ACCENT));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let name: String = app
        .network
        .name
        .as_deref()
        .unwrap_or("Network")
        .chars()
        .filter(|ch| !ch.is_control())
        .collect();
    let signal = app
        .network
        .signal
        .map_or("Signal unavailable".into(), |value| {
            format!("{}  {value} dBm", signal_bars(value))
        });
    let gateway = app
        .network
        .router
        .map_or("not found".into(), |address| address.to_string());
    let left = Line::from(vec![
        Span::styled(" whylag  ", Style::new().fg(ACCENT).bold()),
        Span::styled(name, Style::new().bold()),
        Span::raw("  "),
        Span::styled(signal, Style::new().fg(signal_color(app.network.signal))),
    ]);
    let right = Line::from(vec![
        Span::styled("Gateway ", Style::new().fg(LABEL)),
        Span::raw(format!("{gateway} ")),
    ])
    .right_aligned();
    // Draw the right side first so a long name stays readable.
    frame.render_widget(Paragraph::new(right), inner);
    frame.render_widget(Paragraph::new(left), inner);
}

/// Grid of numbers for the charted target plus general checks.
fn details(frame: &mut Frame, area: Rect, app: &App, stats: &Stats) {
    let block = Block::bordered()
        .title(Line::styled(" Details ", Style::new().fg(LABEL).bold()))
        .border_style(Style::new().fg(ACCENT));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let elapsed = app.elapsed();
    let latest = stats.history.back().copied().flatten();
    let (other_name, other) = if app.show_router {
        ("Internet", &app.internet)
    } else {
        ("Router", &app.router)
    };
    let other_latest = other.history.back().copied().flatten();
    let (dns, dns_color) = match app.dns {
        Some(true) => ("OK", Color::Green),
        Some(false) => ("Failing", Color::Red),
        None => ("Checking", LABEL),
    };
    let loss = stats.loss();
    let cells = [
        ("Latency", milliseconds(latest), latency_color(latest)),
        (
            "Average",
            milliseconds(stats.average()),
            latency_color(stats.average()),
        ),
        ("Best", milliseconds(stats.best), latency_color(stats.best)),
        (
            "Worst",
            milliseconds(stats.worst),
            latency_color(stats.worst),
        ),
        (
            "Jitter",
            milliseconds(stats.jitter()),
            latency_color(stats.jitter()),
        ),
        ("Loss", format!("{loss:.1}%"), good_or(loss > 0.0, ORANGE)),
        (
            "Drops",
            stats.drops.to_string(),
            good_or(stats.drops > 0, Color::Red),
        ),
        ("Offline", clock(stats.offline(elapsed)), Color::Reset),
        (
            other_name,
            milliseconds(other_latest),
            latency_color(other_latest),
        ),
        ("DNS", dns.to_owned(), dns_color),
        ("Low wifi", clock(app.weak_time()), Color::Reset),
        ("Session", clock(elapsed), Color::Reset),
    ];

    // Four columns, filled row by row.
    let columns: [Rect; 4] = Layout::horizontal([Constraint::Ratio(1, 4); 4]).areas(inner);
    for (index, (label, value, color)) in cells.into_iter().enumerate() {
        let column = columns[index % 4];
        let row = (index / 4) as u16;
        if row >= column.height {
            break;
        }
        let line = Line::from(vec![
            Span::styled(format!(" {label:<9}"), Style::new().fg(LABEL)),
            Span::styled(value, Style::new().fg(color).bold()),
        ]);
        let cell = Rect {
            y: column.y + row,
            height: 1,
            ..column
        };
        frame.render_widget(Paragraph::new(line), cell);
    }
}

/// Green when fine, otherwise the given warning color.
fn good_or(problem: bool, color: Color) -> Color {
    if problem { color } else { Color::Green }
}

/// One line that says what is wrong, if anything.
fn status(frame: &mut Frame, area: Rect, app: &App) {
    let health = app.health();
    let color = match health.health {
        Health::Checking => LABEL,
        Health::Good => Color::Green,
        Health::Warning => ORANGE,
        Health::Bad => Color::Red,
    };
    let line = Line::from(vec![
        Span::styled(" ● ", Style::new().fg(color)),
        Span::styled(health.message, Style::new().fg(color).bold()),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

/// The most recent events with their time stamps.
fn events(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::TOP)
        .title(Line::styled(" Events ", Style::new().fg(LABEL).bold()))
        .border_style(Style::new().fg(ACCENT));
    let lines: Vec<Line> = if app.events.is_empty() {
        vec![Line::from(" No events yet")]
    } else {
        app.events
            .iter()
            .map(|text| {
                let (time, message) = text.split_once("  ").unwrap_or(("", text));
                Line::from(vec![
                    Span::styled(format!(" {time}  "), Style::new().fg(LABEL)),
                    Span::raw(message),
                ])
            })
            .collect()
    };
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

/// Key hints, with a banner while paused.
fn footer(frame: &mut Frame, area: Rect, app: &App) {
    let keys: &[(&str, &str)] = if app.paused {
        &[("p", "Resume"), ("q", "Quit")]
    } else {
        &[
            ("q", "Quit"),
            ("p", "Pause"),
            ("t", "Switch chart"),
            ("c", "Reset"),
        ]
    };
    let mut spans = Vec::new();
    if app.paused {
        spans.push(Span::styled(
            " PAUSED ",
            Style::new().fg(Color::Black).bg(Color::Yellow).bold(),
        ));
    }
    for (key, action) in keys {
        spans.push(Span::styled(
            format!(" {key} "),
            Style::new().fg(LABEL).bold(),
        ));
        spans.push(Span::raw(format!("{action}  ")));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn signal_bars(signal: i32) -> &'static str {
    match signal {
        -50.. => "▂▄▆█",
        -60..=-51 => "▂▄▆_",
        -70..=-61 => "▂▄__",
        -80..=-71 => "▂___",
        _ => "____",
    }
}

fn signal_color(signal: Option<i32>) -> Color {
    match signal {
        Some(-60..) => Color::Green,
        Some(-70..=-61) => ORANGE,
        Some(_) => Color::Red,
        None => Color::Reset,
    }
}

/// Bar chart of recent pings, newest on the right.
fn chart(frame: &mut Frame, area: Rect, stats: &Stats, title: &str) {
    let block = Block::bordered()
        .title(Line::styled(
            format!(" {title} "),
            Style::new().fg(LABEL).bold(),
        ))
        .title_bottom(Line::from(format!(" Scale 0 to {CHART_MAX:.0} ms ")).right_aligned())
        .border_style(Style::new().fg(ACCENT));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.is_empty() {
        return;
    }

    // Show as many bars as fit, keeping the newest.
    let step = BAR_WIDTH + BAR_GAP;
    let slots = usize::from((inner.width + BAR_GAP) / step);
    let visible = stats.history.len().min(slots);
    let start = stats.history.len() - visible;
    let used = (visible as u16 * step).saturating_sub(BAR_GAP);
    let left = inner.right() - used;

    for (slot, (index, ping)) in stats.history.iter().enumerate().skip(start).enumerate() {
        let color = bar_color(stats, index, *ping);
        // A lost ping draws a full bar.
        let value = ping.unwrap_or(CHART_MAX).clamp(0.0, CHART_MAX);
        let eighths = ((value / CHART_MAX) * f64::from(inner.height) * 8.0)
            .ceil()
            .max(1.0) as u16;
        let x = left + slot as u16 * step;
        for row in 0..inner.height {
            let symbol = BLOCKS[usize::from(eighths.saturating_sub(row * 8).min(8))];
            let y = inner.bottom() - 1 - row;
            for column in x..x + BAR_WIDTH {
                frame.buffer_mut()[(column, y)]
                    .set_symbol(symbol)
                    .set_fg(color);
            }
        }
    }
}

/// Color from lost pings in the last 10 samples up to this one.
fn bar_color(stats: &Stats, index: usize, ping: Option<f64>) -> Color {
    let losses = stats
        .history
        .iter()
        .take(index + 1)
        .skip((index + 1).saturating_sub(10))
        .filter(|value| value.is_none())
        .count();
    if ping.is_none() || losses >= 2 {
        Color::Red
    } else if losses == 1 {
        ORANGE
    } else {
        Color::Green
    }
}
