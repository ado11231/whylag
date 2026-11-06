//! Renders the README preview at `assets/preview.svg`.
//!
//! Draws the real screen with sample data into a test terminal, then turns
//! each cell into SVG inside a macOS style window.
//!
//! ```sh
//! cargo run --example preview
//! ```

use ratatui::{Terminal, backend::TestBackend, buffer::Cell, style::Color, style::Modifier};
use std::{
    fmt::Write as _,
    fs,
    time::{Duration, Instant},
};
use whylag::{app::App, helpers::Update, platform::Network, ui};

const COLS: u16 = 100;
const ROWS: u16 = 30;
const CELL_W: f64 = 8.4;
const CELL_H: f64 = 18.0;
const PAD: f64 = 22.0;
const BAR: f64 = 36.0;
const MARGIN: f64 = 28.0;
/// Rows above the app: the prompt line and a blank line.
const PROMPT_ROWS: f64 = 2.0;

const FG: &str = "#e8e8e8";
const BG: &str = "#2c2c30";

fn main() {
    let app = sample_app();
    let mut terminal = Terminal::new(TestBackend::new(COLS, ROWS)).expect("test terminal");
    terminal
        .draw(|frame| ui::draw(frame, &app, "1.1.1.1"))
        .expect("draw screen");
    let svg = render(terminal.backend().buffer().content(), COLS);
    fs::write("assets/preview.svg", svg).expect("write assets/preview.svg");
    println!("wrote assets/preview.svg");
}

/// A five minute session with one short outage.
fn sample_app() -> App {
    let mut app = App::default();
    let at = |app: &mut App, seconds: u64| {
        app.started = Instant::now() - Duration::from_secs(seconds) - Duration::from_millis(200);
    };
    at(&mut app, 0);
    app.update(Update::Network(Network {
        name: Some("HomeWifi".into()),
        signal: Some(-58),
        router: None,
    }));
    app.update(Update::Dns(true));

    let total = 312;
    let mut seed = 7_u64;
    for second in 0..total {
        at(&mut app, second);
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let noise = ((seed >> 33) % 1000) as f64 / 1000.0;
        let from_end = total - second;
        let internet = match from_end {
            22..=25 => None,
            27 | 28 => Some(118.0 + noise * 40.0),
            26 => Some(84.0),
            18..=21 => Some(62.0 + noise * 30.0),
            9 => Some(71.0),
            _ => Some(18.0 + noise * 22.0 + if second % 7 == 0 { 9.0 } else { 0.0 }),
        };
        app.update(Update::Ping {
            router: "192.168.1.1".parse().ok(),
            router_ping: Some(3.0 + noise * 3.0),
            internet_ping: internet,
        });
    }
    at(&mut app, total);
    app
}

fn color(color: Color, fallback: &str) -> String {
    match color {
        Color::Reset => fallback.into(),
        Color::Black => "#1c1c1e".into(),
        Color::Red => "#ff453a".into(),
        Color::Green => "#30d158".into(),
        Color::Yellow => "#ffd60a".into(),
        Color::Magenta => "#bf5af2".into(),
        Color::Cyan => "#64d2ff".into(),
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        _ => fallback.into(),
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Height of a lower block character in eighths of a cell.
fn block_eighths(symbol: &str) -> Option<u8> {
    let index = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"]
        .iter()
        .position(|block| *block == symbol)?;
    Some(index as u8 + 1)
}

/// Which sides of a box drawing character have a line: up, right, down, left.
fn box_sides(symbol: &str) -> Option<[bool; 4]> {
    Some(match symbol {
        "─" => [false, true, false, true],
        "│" => [true, false, true, false],
        "┌" => [false, true, true, false],
        "┐" => [false, false, true, true],
        "└" => [true, true, false, false],
        "┘" => [true, false, false, true],
        _ => return None,
    })
}

fn render(cells: &[Cell], cols: u16) -> String {
    let cols = usize::from(cols);
    let rows = cells.len() / cols;
    let win_w = PAD * 2.0 + cols as f64 * CELL_W;
    let win_h = BAR + 14.0 + (rows as f64 + PROMPT_ROWS + 2.0) * CELL_H + 18.0;
    let width = win_w + MARGIN * 2.0;
    let height = win_h + MARGIN * 2.0;
    let top = BAR + 14.0;
    let baseline = |row: f64| top + row * CELL_H + 13.0;

    let mut svg = String::new();
    let _ = writeln!(
        svg,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0}" height="{height:.0}" viewBox="0 0 {width:.0} {height:.0}" font-family="ui-monospace, SFMono-Regular, Menlo, Consolas, 'DejaVu Sans Mono', monospace" font-size="13.6">
<defs>
<filter id="shadow" x="-10%" y="-10%" width="120%" height="130%"><feDropShadow dx="0" dy="10" stdDeviation="12" flood-color="#000" flood-opacity="0.45"/></filter>
<linearGradient id="bar" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#4a4a4e"/><stop offset="1" stop-color="#3c3c40"/></linearGradient>
<clipPath id="win"><rect width="{win_w:.1}" height="{win_h:.1}" rx="12"/></clipPath>
</defs>
<g transform="translate({MARGIN},{MARGIN})">
<rect width="{win_w:.1}" height="{win_h:.1}" rx="12" fill="{BG}" filter="url(#shadow)"/>
<g clip-path="url(#win)">
<rect width="{win_w:.1}" height="{BAR}" fill="url(#bar)"/>
<line x1="0" y1="{BAR}" x2="{win_w:.1}" y2="{BAR}" stroke="#111" stroke-width="1"/>
</g>
<rect x="0.5" y="0.5" width="{:.1}" height="{:.1}" rx="12" fill="none" stroke="#4a4a4c" stroke-width="1"/>
<circle cx="20" cy="18" r="6.5" fill="#ff5f57"/><circle cx="40" cy="18" r="6.5" fill="#febc2e"/><circle cx="60" cy="18" r="6.5" fill="#28c840"/>
<text x="{:.1}" y="23" fill="#ffffff" text-anchor="middle" font-size="14" font-weight="700" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif">whylag</text>"##,
        win_w - 1.0,
        win_h - 1.0,
        win_w / 2.0,
    );
    prompt(&mut svg, baseline(0.0), Some("whylag"));

    for row in 0..rows {
        let y = top + (row as f64 + PROMPT_ROWS) * CELL_H;
        let mut col = 0;
        while col < cols {
            let cell = &cells[row * cols + col];
            let x = PAD + col as f64 * CELL_W;
            let symbol = cell.symbol();
            let fill = color(cell.fg, FG);
            if cell.bg != Color::Reset {
                let _ = writeln!(
                    svg,
                    r#"<rect x="{x:.2}" y="{y:.2}" width="{:.2}" height="{CELL_H}" fill="{}"/>"#,
                    CELL_W + 0.3,
                    color(cell.bg, BG)
                );
            }
            if let Some(eighths) = block_eighths(symbol) {
                let h = CELL_H * f64::from(eighths) / 8.0;
                let _ = writeln!(
                    svg,
                    r#"<rect x="{x:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{fill}"/>"#,
                    y + CELL_H - h - 0.4,
                    CELL_W + 0.3,
                    h + 0.4,
                );
                col += 1;
                continue;
            }
            if let Some([up, right, down, left]) = box_sides(symbol) {
                let (cx, cy) = (x + CELL_W / 2.0, y + CELL_H / 2.0);
                let mut line = |x1: f64, y1: f64, x2: f64, y2: f64| {
                    let _ = writeln!(
                        svg,
                        r#"<line x1="{x1:.2}" y1="{y1:.2}" x2="{x2:.2}" y2="{y2:.2}" stroke="{fill}" stroke-width="1.1"/>"#
                    );
                };
                if left || right {
                    let x1 = if left { x } else { cx };
                    let x2 = if right { x + CELL_W } else { cx };
                    line(x1, cy, x2, cy);
                }
                if up || down {
                    let y1 = if up { y } else { cy };
                    let y2 = if down { y + CELL_H } else { cy };
                    line(cx, y1, cx, y2);
                }
                col += 1;
                continue;
            }
            // Group plain text with the same style into one element.
            let start = col;
            let mut text = String::new();
            while col < cols {
                let next = &cells[row * cols + col];
                let sym = next.symbol();
                if next.fg != cell.fg
                    || next.modifier != cell.modifier
                    || next.bg != cell.bg
                    || block_eighths(sym).is_some()
                    || box_sides(sym).is_some()
                {
                    break;
                }
                text.push_str(sym);
                col += 1;
            }
            if text.trim().is_empty() {
                continue;
            }
            let weight = if cell.modifier.contains(Modifier::BOLD) {
                "700"
            } else {
                "normal"
            };
            let _ = writeln!(
                svg,
                r#"<text x="{:.1}" y="{:.1}" fill="{fill}" font-weight="{weight}" xml:space="preserve">{}</text>"#,
                PAD + start as f64 * CELL_W,
                y + 13.0,
                escape(&text)
            );
        }
    }

    prompt(&mut svg, baseline(rows as f64 + PROMPT_ROWS + 1.0), None);
    svg.push_str("</g>\n</svg>\n");
    svg
}

/// A shell prompt, with a command or a cursor.
fn prompt(svg: &mut String, y: f64, command: Option<&str>) {
    let _ = writeln!(
        svg,
        r##"<text x="{PAD}" y="{y:.1}" fill="#64d2ff">~</text>
<text x="{:.1}" y="{y:.1}" fill="#30d158">❯</text>"##,
        PAD + CELL_W * 2.0
    );
    let x = PAD + CELL_W * 4.0;
    match command {
        Some(command) => {
            let _ = writeln!(
                svg,
                r#"<text x="{x:.1}" y="{y:.1}" fill="{FG}">{command}</text>"#
            );
        }
        None => {
            let _ = writeln!(
                svg,
                r#"<rect x="{x:.1}" y="{:.1}" width="8.4" height="14.0" fill="{FG}" opacity="0.8"/>"#,
                y - 11.0
            );
        }
    }
}
