use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::{
    io::{self, IsTerminal},
    process::ExitCode,
    time::Duration,
};
use whylag::{
    app::App,
    helpers::{Helpers, Update},
    log::Log,
    options::{self, Action, Options},
    summary, ui,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("whylag: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let options = match options::parse(std::env::args_os().skip(1))? {
        Action::Help => {
            println!("{}", options::HELP);
            return Ok(());
        }
        Action::Version => {
            println!("whylag {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Action::Run(options) => options,
    };
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("an interactive terminal is required; use --help for options".into());
    }
    whylag::command::check_tools(options.target.is_ipv6())?;
    let mut log = options.log.as_deref().map(Log::create).transpose()?;
    let mut app = App::default();
    let (helpers, updates) = Helpers::start(options.target);
    let mut terminal = ratatui::init();
    let result = monitor(&mut terminal, &mut app, &updates, &options, &mut log);
    ratatui::restore();
    drop(helpers);
    result?;
    summary::print(&app)?;
    Ok(())
}

fn monitor(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    updates: &std::sync::mpsc::Receiver<Update>,
    options: &Options,
    log: &mut Option<Log>,
) -> io::Result<()> {
    let target = options.target.to_string();
    loop {
        for update in updates.try_iter() {
            if let Update::Ping {
                router,
                router_ping,
                internet_ping,
            } = &update
                && let Some(log) = log
            {
                log.record(options.target, *router, *router_ping, *internet_ping)?;
            }
            app.update(update);
        }
        if !app.paused {
            terminal.draw(|frame| ui::draw(frame, app, &target))?;
        }
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char('p') => {
                        app.paused = !app.paused;
                        terminal.draw(|frame| ui::draw(frame, app, &target))?;
                    }
                    KeyCode::Char('t') => app.show_router = !app.show_router,
                    KeyCode::Char('c') => app.clear(),
                    _ => {}
                },
                Event::Resize(_, _) => {
                    terminal.draw(|frame| ui::draw(frame, app, &target))?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
