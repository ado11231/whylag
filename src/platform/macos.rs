use super::Network;
use crate::command;
use std::{net::IpAddr, time::Duration};

pub fn ping_args() -> Vec<&'static str> {
    vec!["-n", "-c", "1", "-W", "1000"]
}

pub fn get_router() -> Option<IpAddr> {
    let output =
        command::output("route", &["-n", "get", "default"], Duration::from_secs(2)).ok()?;
    output.lines().find_map(|line| {
        line.trim()
            .strip_prefix("gateway:")
            .and_then(|value| value.trim().parse().ok())
    })
}

pub fn read_network() -> Network {
    let router = get_router();
    let output = command::output(
        "system_profiler",
        &["SPAirPortDataType"],
        Duration::from_secs(4),
    )
    .unwrap_or_default();
    let (name, signal) = parse_wifi(&output);
    Network {
        name,
        signal,
        router,
    }
}

pub fn parse_wifi(output: &str) -> (Option<String>, Option<i32>) {
    let Some((_, current)) = output.split_once("Current Network Information:") else {
        return (None, None);
    };
    let name = current
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let name = line.trim().trim_end_matches(':');
            if name.to_lowercase().contains("redacted") {
                "Wi-Fi".to_owned()
            } else {
                name.to_owned()
            }
        });
    let signal = current
        .lines()
        .take_while(|line| !line.contains("Other Local Wi-Fi Networks:"))
        .find_map(|line| {
            line.trim()
                .strip_prefix("Signal / Noise:")?
                .split_whitespace()
                .next()?
                .parse()
                .ok()
        });
    (name, signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_only_current_network() {
        let text = "Current Network Information:\n  <redacted>:\n    Signal / Noise: -67 dBm / -90 dBm\nOther Local Wi-Fi Networks:\n  Other:";
        assert_eq!(parse_wifi(text), (Some("Wi-Fi".into()), Some(-67)));
        assert_eq!(parse_wifi("Wi-Fi: off"), (None, None));
    }
}
