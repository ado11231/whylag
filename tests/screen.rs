use ratatui::{Terminal, backend::TestBackend, style::Color};
use whylag::{app::App, helpers::Update, ui};

fn sample(app: &mut App, ping: Option<f64>) {
    app.update(Update::Ping {
        router: "192.168.1.1".parse().ok(),
        router_ping: Some(4.0),
        internet_ping: ping,
    });
}

#[test]
fn renders_status_and_handles_small_terminals() {
    let mut app = App::default();
    sample(&mut app, Some(25.0));
    app.update(Update::Dns(true));
    for (width, height) in [(80, 24), (42, 16), (20, 8), (1, 1)] {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("test terminal");
        terminal
            .draw(|frame| ui::draw(frame, &app, "1.1.1.1"))
            .expect("draw screen");
        let buffer = terminal.backend().buffer();
        let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
        if width >= 42 {
            assert!(text.contains("All good"));
        } else if width > 1 {
            assert!(text.contains("Resize"));
        }
    }
}

#[test]
fn lost_ping_has_a_full_red_bar() {
    let mut app = App::default();
    sample(&mut app, None);
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("test terminal");
    terminal
        .draw(|frame| ui::draw(frame, &app, "1.1.1.1"))
        .expect("draw screen");
    let buffer = terminal.backend().buffer();
    let red_bars = buffer
        .content
        .iter()
        .filter(|cell| cell.symbol() == "█" && cell.fg == Color::Red)
        .count();
    assert!(red_bars >= 4, "lost samples should fill the chart height");
}

#[test]
fn clear_resets_stats_and_preserves_network_details() {
    let mut app = App::default();
    for _ in 0..3 {
        sample(&mut app, None);
    }
    assert_eq!(app.internet.drops, 1);
    app.clear();
    assert_eq!(app.internet.sent, 0);
    assert_eq!(app.internet.drops, 0);
    assert!(app.network.router.is_some());
    assert_eq!(app.events.len(), 1);
}

#[test]
fn dns_events_only_record_transitions() {
    let mut app = App::default();
    for _ in 0..20 {
        app.update(Update::Dns(false));
    }
    assert_eq!(app.events.len(), 1);
    app.update(Update::Dns(true));
    assert_eq!(app.events.len(), 2);
    assert!(
        app.events
            .back()
            .is_some_and(|text| text.contains("DNS recovered"))
    );
}
