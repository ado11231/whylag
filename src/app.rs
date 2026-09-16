use crate::{
    helpers::Update,
    platform::Network,
    stats::Stats,
    status::{self, Status},
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct App {
    pub started: Instant,
    finished: Option<Duration>,
    pub router: Stats,
    pub internet: Stats,
    pub network: Network,
    pub dns: Option<bool>,
    pub events: VecDeque<String>,
    pub show_router: bool,
    pub paused: bool,
    weak_started: Option<Duration>,
    weak_total: Duration,
}

impl Default for App {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            finished: None,
            router: Stats::default(),
            internet: Stats::default(),
            network: Network::default(),
            dns: None,
            events: VecDeque::new(),
            show_router: false,
            paused: false,
            weak_started: None,
            weak_total: Duration::ZERO,
        }
    }
}

impl App {
    pub fn elapsed(&self) -> Duration {
        self.finished.unwrap_or_else(|| self.started.elapsed())
    }

    pub fn finish(&mut self) {
        self.finished = Some(self.elapsed());
    }

    pub fn update(&mut self, update: Update) {
        let elapsed = self.elapsed();
        match update {
            Update::Ping {
                router,
                router_ping,
                internet_ping,
            } => {
                self.network.router = router;
                let was_down = self.internet.is_down();
                let offline = self.internet.current_outage(elapsed);
                self.router.record(router_ping, elapsed);
                self.internet.record(internet_ping, elapsed);
                if !was_down && self.internet.is_down() {
                    self.event("Lost connection".into());
                }
                if was_down && !self.internet.is_down() {
                    self.event(format!("Back online after {}s", offline.as_secs()));
                }
            }
            Update::Network(network) => {
                if self.network.name != network.name
                    && let Some(name) = &network.name
                {
                    self.event(format!("Joined {name}"));
                }
                let weak = network.signal.is_some_and(|value| value <= -71);
                if weak && self.weak_started.is_none() {
                    self.weak_started = Some(elapsed);
                    self.event("Signal dropped to 1 bar".into());
                } else if !weak && let Some(start) = self.weak_started.take() {
                    self.weak_total += elapsed.saturating_sub(start);
                    if network.signal.is_some() {
                        self.event("Signal recovered".into());
                    }
                }
                self.network.name = network.name;
                self.network.signal = network.signal;
            }
            Update::Dns(healthy) => {
                if self.dns != Some(healthy) && (self.dns.is_some() || !healthy) {
                    self.event(
                        if healthy {
                            "DNS recovered"
                        } else {
                            "DNS lookup failed"
                        }
                        .into(),
                    );
                }
                self.dns = Some(healthy);
            }
        }
    }

    fn event(&mut self, message: String) {
        if self.events.len() == 5 {
            self.events.pop_front();
        }
        let seconds = self.elapsed().as_secs();
        let clean: String = message.chars().filter(|ch| !ch.is_control()).collect();
        self.events
            .push_back(format!("{:02}:{:02}  {clean}", seconds / 60, seconds % 60));
    }

    pub fn health(&self) -> Status {
        status::evaluate(
            self.network.router.is_some(),
            self.router.history.back().copied(),
            self.internet.history.back().copied(),
            self.dns,
            self.network.signal,
            self.internet.recent_loss(),
        )
    }

    pub fn weak_time(&self) -> Duration {
        self.weak_total
            + self
                .weak_started
                .map_or(Duration::ZERO, |start| self.elapsed().saturating_sub(start))
    }

    pub fn clear(&mut self) {
        self.router = Stats::default();
        self.internet = Stats::default();
        self.events.clear();
        self.started = Instant::now();
        self.finished = None;
        self.weak_total = Duration::ZERO;
        self.weak_started = self
            .network
            .signal
            .filter(|value| *value <= -71)
            .map(|_| Duration::ZERO);
        self.event("Stats cleared".into());
    }
}
