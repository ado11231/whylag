use crate::{
    app::App,
    ui::{clock, latency_color, milliseconds},
};
use ratatui::style::Color;
use std::io::{self, IsTerminal, Write};

pub fn print(app: &App) -> io::Result<()> {
    let stdout = io::stdout();
    let color = stdout.is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let mut out = stdout.lock();
    let (border, label, reset) = if color {
        ("\x1b[35m", "\x1b[36m", "\x1b[0m")
    } else {
        ("", "", "")
    };
    let stats = &app.internet;
    writeln!(
        out,
        "{border}╭─ whylag summary ───────────────────────────╮{reset}"
    )?;
    let elapsed = app.elapsed();
    for (name, value) in [
        ("Session", clock(elapsed)),
        ("Average", milliseconds(stats.average())),
        (
            "Best / Worst",
            format!(
                "{} / {}",
                milliseconds(stats.best),
                milliseconds(stats.worst)
            ),
        ),
        ("Packet loss", format!("{:.1}%", stats.loss())),
        ("Jitter", milliseconds(stats.jitter())),
        (
            "Drops",
            format!(
                "{} ({} offline)",
                stats.drops,
                clock(stats.offline(elapsed))
            ),
        ),
        ("Low wifi", clock(app.weak_time())),
    ] {
        let number = if color {
            match name {
                "Average" => ansi(latency_color(stats.average())),
                "Jitter" => ansi(latency_color(stats.jitter())),
                "Packet loss" if stats.loss() > 0.0 => "\x1b[33m",
                "Drops" if stats.drops > 0 => "\x1b[31m",
                _ => "",
            }
        } else {
            ""
        };
        writeln!(
            out,
            "{border}│{reset} {label}{name:<15}{reset} {number}{value:<25}{reset} {border}│{reset}"
        )?;
    }
    writeln!(
        out,
        "{border}╰───────────────────────────────────────────╯{reset}"
    )
}

fn ansi(color: Color) -> &'static str {
    match color {
        Color::Green => "\x1b[32m",
        Color::Red => "\x1b[31m",
        Color::Reset => "",
        _ => "\x1b[33m",
    }
}
