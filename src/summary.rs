use crate::{
    app::App,
    ui::{latency_color, milliseconds},
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
    let elapsed = app.started.elapsed();
    for (name, value) in [
        (
            "Duration",
            format!("{}m {}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60),
        ),
        ("Average ping", milliseconds(stats.average())),
        (
            "Best / Worst",
            format!(
                "{} / {}",
                milliseconds(stats.best),
                milliseconds(stats.worst)
            ),
        ),
        ("Loss", format!("{:.1}%", stats.loss())),
        ("Jitter", milliseconds(stats.jitter())),
        (
            "Drops",
            format!(
                "{} ({}s offline)",
                stats.drops,
                stats.offline(elapsed).as_secs()
            ),
        ),
        ("Weak signal", format!("{}s", app.weak_time().as_secs())),
    ] {
        let number = if color {
            match name {
                "Average ping" => ansi(latency_color(stats.average())),
                "Jitter" => ansi(latency_color(stats.jitter())),
                "Loss" if stats.loss() > 0.0 => "\x1b[33m",
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
        _ => "\x1b[33m",
    }
}
