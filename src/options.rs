use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

pub const HELP: &str = "whylag\nLive wifi and internet checks.\n\nUsage: whylag [--target <IP address>] [--log <file>]\n\n  --target  Address to ping (default: 1.1.1.1)\n  --log     Save each sample to a new CSV file\n  -h, --help  Show this help\n  -V, --version  Show the version\n\nKeys: q quit  p pause  t switch chart  c reset stats";

pub struct Options {
    pub target: IpAddr,
    pub log: Option<PathBuf>,
}
pub enum Action {
    Run(Options),
    Help,
    Version,
}

pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Action, String> {
    let mut args = args.into_iter();
    let mut options = Options {
        target: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        log: None,
    };
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("-h" | "--help") => return Ok(Action::Help),
            Some("-V" | "--version") => return Ok(Action::Version),
            Some("--target") => {
                let value = args.next().ok_or("--target needs an IP address")?;
                options.target = value
                    .to_str()
                    .and_then(|value| value.parse().ok())
                    .ok_or("--target must be an IPv4 or IPv6 address")?;
            }
            Some("--log") => {
                options.log = Some(
                    args.next()
                        .filter(|value| !value.is_empty())
                        .ok_or("--log needs a file path")?
                        .into(),
                )
            }
            _ => return Err(format!("unknown option: {}", arg.to_string_lossy())),
        }
    }
    Ok(Action::Run(options))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_and_invalid_arguments() {
        for args in [
            vec!["--target"],
            vec!["--target", "-c"],
            vec!["--log"],
            vec!["--unknown"],
        ] {
            assert!(parse(args.into_iter().map(OsString::from)).is_err());
        }
    }
    #[test]
    fn accepts_combined_options() {
        let action = parse(["--target", "8.8.8.8", "--log", "pings.csv"].map(OsString::from))
            .expect("valid options");
        let Action::Run(options) = action else {
            panic!("expected run options")
        };
        assert_eq!(options.target.to_string(), "8.8.8.8");
        assert_eq!(options.log, Some(PathBuf::from("pings.csv")));
    }
}
