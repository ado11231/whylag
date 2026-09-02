use super::Network;
use crate::command;
use std::{net::IpAddr, time::Duration};

pub fn ping_args() -> Vec<&'static str> {
    vec!["-n", "-c", "1", "-W", "1"]
}

pub fn get_router() -> Option<IpAddr> {
    let output =
        command::output("ip", &["route", "show", "default"], Duration::from_secs(2)).ok()?;
    parse_router(&output)
}

fn parse_router(output: &str) -> Option<IpAddr> {
    output.lines().find_map(|line| {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.first() != Some(&"default") {
            return None;
        }
        words.windows(2).find(|pair| pair[0] == "via")?[1]
            .parse()
            .ok()
    })
}

pub fn read_network() -> Network {
    let router = get_router();
    let wireless = std::fs::read_to_string("/proc/net/wireless").unwrap_or_default();
    let interface = wireless
        .lines()
        .find_map(|line| line.split_once(':').map(|(name, _)| name.trim()));
    let signal = get_signal(&wireless);
    let name = interface.and_then(|interface| {
        command::output("iw", &["dev", interface, "link"], Duration::from_secs(2))
            .ok()
            .and_then(|text| {
                text.lines()
                    .find_map(|line| line.trim().strip_prefix("SSID: ").map(str::to_owned))
            })
    });
    Network {
        name,
        signal,
        router,
    }
}

fn get_signal(wireless: &str) -> Option<i32> {
    wireless.lines().find_map(|line| {
        let (_, fields) = line.split_once(':')?;
        fields
            .split_whitespace()
            .nth(2)?
            .trim_end_matches('.')
            .parse()
            .ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_default_gateway_and_wireless_signal() {
        assert_eq!(
            parse_router("default via 192.168.1.1 dev wlan0 metric 600"),
            "192.168.1.1".parse().ok()
        );
        assert_eq!(parse_router("10.0.0.0/24 dev eth0"), None);
        assert_eq!(get_signal("wlan0: 0000 54. -56. -256 0 0"), Some(-56));
        assert_eq!(get_signal("Inter-| sta-| Quality"), None);
    }
}
