use std::collections::VecDeque;
use std::time::Duration;

#[derive(Default)]
pub struct Stats {
    pub history: VecDeque<Option<f64>>,
    pub sent: u64,
    pub received: u64,
    pub best: Option<f64>,
    pub worst: Option<f64>,
    pub drops: u64,
    total: f64,
    change_total: f64,
    changes: u64,
    previous: Option<f64>,
    failures: u64,
    failure_start: Option<Duration>,
    offline: Duration,
}

impl Stats {
    pub fn record(&mut self, ping: Option<f64>, elapsed: Duration) {
        self.sent += 1;
        if self.history.len() == 300 {
            self.history.pop_front();
        }
        self.history.push_back(ping);
        if let Some(value) = ping {
            self.received += 1;
            self.total += value;
            self.best = Some(self.best.map_or(value, |best| best.min(value)));
            self.worst = Some(self.worst.map_or(value, |worst| worst.max(value)));
            if let Some(previous) = self.previous {
                self.change_total += (value - previous).abs();
                self.changes += 1;
            }
            if self.failures >= 3
                && let Some(start) = self.failure_start
            {
                self.offline += elapsed.saturating_sub(start);
            }
            self.failures = 0;
            self.failure_start = None;
        } else {
            if self.failures == 0 {
                self.failure_start = Some(elapsed);
            }
            self.failures += 1;
            if self.failures == 3 {
                self.drops += 1;
            }
        }
        self.previous = ping;
    }

    pub fn average(&self) -> Option<f64> {
        (self.received > 0).then(|| self.total / self.received as f64)
    }

    pub fn jitter(&self) -> Option<f64> {
        (self.changes > 0).then(|| self.change_total / self.changes as f64)
    }

    pub fn loss(&self) -> f64 {
        if self.sent == 0 {
            0.0
        } else {
            100.0 * (self.sent - self.received) as f64 / self.sent as f64
        }
    }

    pub fn recent_loss(&self) -> usize {
        self.history
            .iter()
            .rev()
            .take(10)
            .filter(|ping| ping.is_none())
            .count()
    }

    pub fn is_down(&self) -> bool {
        self.failures >= 3
    }

    pub fn current_outage(&self, elapsed: Duration) -> Duration {
        self.failure_start
            .filter(|_| self.is_down())
            .map_or(Duration::ZERO, |start| elapsed.saturating_sub(start))
    }

    pub fn offline(&self, elapsed: Duration) -> Duration {
        let current = self
            .failure_start
            .filter(|_| self.is_down())
            .map_or(Duration::ZERO, |start| elapsed.saturating_sub(start));
        self.offline + current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loss_does_not_count_as_zero_latency() {
        let mut stats = Stats::default();
        for (second, ping) in [Some(10.0), Some(30.0), None, Some(50.0)]
            .into_iter()
            .enumerate()
        {
            stats.record(ping, Duration::from_secs(second as u64));
        }
        assert_eq!(stats.average(), Some(30.0));
        assert_eq!(stats.jitter(), Some(20.0));
        assert_eq!(stats.loss(), 25.0);
        assert_eq!(stats.best, Some(10.0));
        assert_eq!(stats.worst, Some(50.0));
    }

    #[test]
    fn outage_counts_once_and_includes_first_failures() {
        let mut stats = Stats::default();
        for second in 1..=5 {
            stats.record(None, Duration::from_secs(second));
        }
        assert_eq!(stats.drops, 1);
        assert_eq!(stats.offline(Duration::from_secs(6)).as_secs(), 5);
        stats.record(Some(4.0), Duration::from_secs(7));
        assert!(!stats.is_down());
        assert_eq!(stats.offline(Duration::from_secs(10)).as_secs(), 6);
    }

    #[test]
    fn history_is_bounded_without_losing_session_totals() {
        let mut stats = Stats::default();
        for second in 0..400 {
            stats.record(Some(5.0), Duration::from_secs(second));
        }
        assert_eq!(stats.history.len(), 300);
        assert_eq!(stats.sent, 400);
        assert_eq!(Stats::default().average(), None);
    }
}
