# whylag

* Live wifi & internet checks in the terminal.
* Shows router ping, internet ping, loss & DNS problems.
* Built for Mac & Linux. No admin rights needed.

## Install

```sh
cargo install --path . --locked
whylag
```

* Needs Rust 1.88 or newer & a terminal at least 60 columns by 22 rows.
* Mac uses built-in commands. Wifi details may be hidden by macOS.
* Linux needs `ping`, `ip` & `getent`. Install `iw` for the network name.
* Missing signal details show as unavailable.

## Use

```sh
whylag --target 8.8.8.8
whylag --log pings.csv
```

* `q` or `Ctrl+C`: quit & show the summary.
* `p`: pause the screen. Checks & logging keep running.
* `t`: switch between internet & router charts.
* `c`: reset stats & start a new session.
* Log files must be new. Existing files are never replaced.
* A missed ping can mean filtering or a busy router, not just an outage.

## Development

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## Docs

* [Architecture](docs/ARCHITECTURE.md)
* [Screen](docs/SCREEN.md)
* [Commands](docs/COMMANDS.md)
