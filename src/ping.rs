use crate::{command, platform};
use std::{net::IpAddr, time::Duration};

pub fn ping(address: IpAddr) -> Option<f64> {
    let target = address.to_string();
    let mut args = platform::ping_args();
    let mut program = "ping";
    if address.is_ipv6() {
        if cfg!(target_os = "macos") {
            program = "ping6";
            args = vec!["-n", "-c", "1"];
        } else {
            args.push("-6");
        }
    }
    args.push(&target);
    command::output(program, &args, Duration::from_millis(1500))
        .ok()
        .and_then(|text| parse_time(&text))
}

pub fn parse_time(text: &str) -> Option<f64> {
    let rest = text
        .split_once("time=")
        .or_else(|| text.split_once("time<"))?
        .1;
    let value: f64 = rest.split_whitespace().next()?.parse().ok()?;
    (value.is_finite() && value >= 0.0).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_linux_and_mac_replies() {
        assert_eq!(parse_time("64 bytes: ttl=64 time=3.52 ms"), Some(3.52));
        assert_eq!(parse_time("64 bytes: time<1 ms"), Some(1.0));
        for text in [
            "Request timeout",
            "time=NaN ms",
            "time=-1 ms",
            "time=inf ms",
        ] {
            assert_eq!(parse_time(text), None);
        }
    }
}
