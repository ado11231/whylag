use crate::{app::App, ui::milliseconds};
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
        writeln!(
            out,
            "{border}│{reset} {label}{name:<15}{reset} {value:<25} {border}│{reset}"
        )?;
    }
    writeln!(
        out,
        "{border}╰───────────────────────────────────────────╯{reset}"
    )
}
