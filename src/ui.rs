use crate::{app::App, stats::Stats, status::Health};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub const ORANGE: Color = Color::Rgb(235, 163, 65);
pub fn latency_color(ping: Option<f64>) -> Color {
    match ping {
        Some(value) if value < 50.0 => Color::Green,
        Some(value) if value <= 150.0 => ORANGE,
        _ => Color::Red,
    }
}
pub fn milliseconds(value: Option<f64>) -> String {
    value.map_or("--".into(), |value| format!("{value:.0} ms"))
}

pub fn draw(frame: &mut Frame, app: &App, target: &str) {
    let area = frame.area();
    if area.width < 42 || area.height < 18 {
        frame.render_widget(
            Paragraph::new("whylag\nResize to at least 42 x 18\nq quit").cyan(),
            area,
        );
        return;
    }
    let sections = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(4),
        Constraint::Length(3),
        Constraint::Length(6),
        Constraint::Length(1),
    ])
    .split(area);
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
        .map_or("signal unavailable".into(), |value| {
            format!("{}  {value} dBm", signal_bars(value))
        });
    let header = Line::from(vec![
        Span::styled(" whylag  ", Style::new().magenta().bold()),
        Span::raw(name),
        Span::raw("  "),
        Span::styled(signal, Style::new().fg(signal_color(app.network.signal))),
    ]);
    frame.render_widget(
        Paragraph::new(header).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::new().dark_gray()),
        ),
        sections[0],
    );
    let (stats, label) = if app.show_router {
        (&app.router, "Router".to_owned())
    } else {
        (&app.internet, format!("Internet · {target}"))
    };
    chart(frame, sections[1], stats, &label);
    let health = app.health();
    let color = match health.health {
        Health::Checking => Color::Gray,
        Health::Good => Color::Green,
        Health::Warning => ORANGE,
        Health::Bad => Color::Red,
    };
    let stats_line = Line::from(vec![
        Span::styled(" Router ", Style::new().cyan()),
        Span::raw(milliseconds(app.router.history.back().copied().flatten())),
        Span::styled("  Net ", Style::new().cyan()),
        Span::raw(milliseconds(app.internet.history.back().copied().flatten())),
        Span::raw(format!(
            "  Loss {:.1}%  Drops {}",
            app.internet.loss(),
            app.internet.drops
        )),
    ]);
    frame.render_widget(
        Paragraph::new(vec![
            stats_line,
            Line::styled(format!(" {}", health.message), Style::new().fg(color)),
        ]),
        sections[2],
    );
    let event_lines: Vec<_> = app
        .events
        .iter()
        .rev()
        .take(5)
        .rev()
        .map(|text| Line::from(format!(" {text}")))
        .collect();
    frame.render_widget(
        Paragraph::new(event_lines)
            .style(Style::new().gray())
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(" Events ")
                    .border_style(Style::new().dark_gray()),
            ),
        sections[3],
    );
    let footer = if app.paused {
        " PAUSED  p resume  q quit"
    } else {
        " q quit   p pause   t swap   c clear"
    };
    frame.render_widget(
        Paragraph::new(footer).style(Style::new().fg(if app.paused {
            Color::Yellow
        } else {
            Color::Gray
        })),
        sections[4],
    );
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
        None => Color::Gray,
    }
}

fn chart(frame: &mut Frame, area: Rect, stats: &Stats, label: &str) {
    let block = Block::bordered()
        .title(format!(" {label} ping "))
        .title_bottom(" 0–300 ms · [t] swap ")
        .border_style(Style::new().dark_gray());
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.is_empty() {
        return;
    }
    let visible = stats.history.len().min(inner.width as usize);
    let start = stats.history.len() - visible;
    let offset = inner.width as usize - visible;
    for (column, index) in (start..stats.history.len()).enumerate() {
        let ping = stats.history[index];
        let losses = stats
            .history
            .iter()
            .take(index + 1)
            .skip((index + 1).saturating_sub(10))
            .filter(|value| value.is_none())
            .count();
        let color = if ping.is_none() || losses >= 2 {
            Color::Red
        } else if losses == 1 {
            ORANGE
        } else {
            Color::Green
        };
        let eighths =
            ((ping.unwrap_or(300.0).clamp(0.0, 300.0) / 300.0) * f64::from(inner.height) * 8.0)
                .ceil()
                .max(1.0) as u16;
        for row in 0..inner.height {
            let fill = eighths.saturating_sub(row * 8).min(8);
            let symbol = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"][fill as usize];
            frame.buffer_mut()[(inner.x + (offset + column) as u16, inner.bottom() - 1 - row)]
                .set_symbol(symbol)
                .set_fg(color);
        }
    }
}
