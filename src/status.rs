#[derive(Debug, PartialEq, Eq)]
pub enum Health {
    Checking,
    Good,
    Warning,
    Bad,
}

pub struct Status {
    pub message: &'static str,
    pub health: Health,
}

pub fn evaluate(
    router_found: bool,
    router: Option<Option<f64>>,
    internet: Option<Option<f64>>,
    dns: Option<bool>,
    signal: Option<i32>,
    recent_loss: usize,
) -> Status {
    // The first matching check decides the message.
    let (message, health) = if router.is_none() {
        ("Checking connection", Health::Checking)
    } else if !router_found {
        ("Disconnected: no router found", Health::Bad)
    } else if router == Some(None) {
        ("Router unreachable: check your wifi", Health::Bad)
    } else if internet == Some(None) {
        ("Internet unreachable: check your provider", Health::Bad)
    } else if dns == Some(false) {
        ("DNS failing: websites may not load", Health::Bad)
    } else if signal.is_some_and(|value| value <= -71) {
        ("Low wifi signal", Health::Warning)
    } else if router.flatten().is_some_and(|value| value > 50.0) {
        ("High router latency", Health::Warning)
    } else if internet.flatten().is_some_and(|value| value > 150.0) {
        ("High internet latency", Health::Warning)
    } else if recent_loss > 0 {
        ("Packet loss detected", Health::Warning)
    } else if internet.is_none() || dns.is_none() {
        ("Checking connection", Health::Checking)
    } else {
        ("Connection healthy", Health::Good)
    };
    Status { message, health }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pending_checks_do_not_report_success() {
        assert_eq!(
            evaluate(true, Some(Some(4.0)), Some(Some(20.0)), None, None, 0).health,
            Health::Checking
        );
    }
    #[test]
    fn router_failure_takes_priority_over_dns() {
        let status = evaluate(true, Some(None), Some(None), Some(false), Some(-80), 2);
        assert_eq!(status.message, "Router unreachable: check your wifi");
    }
    #[test]
    fn good_connection_can_have_unknown_signal() {
        assert_eq!(
            evaluate(true, Some(Some(4.0)), Some(Some(20.0)), Some(true), None, 0).health,
            Health::Good
        );
    }
}
