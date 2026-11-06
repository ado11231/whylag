<div align="center">

<h3>whylag</h3>

see why your internet is lagging, live in your terminal

[![Crates.io](https://img.shields.io/crates/v/whylag.svg)](https://crates.io/crates/whylag)
[![CI](https://github.com/ado11231/whylag/actions/workflows/check.yml/badge.svg)](https://github.com/ado11231/whylag/actions/workflows/check.yml)
[![MSRV](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux-lightgrey.svg)](#install)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<img src="assets/preview.svg" alt="whylag showing a latency bar chart, connection details and recent events" width="100%">

</div>

## Install

```sh
cargo install whylag
```

Needs Rust 1.88+ and a terminal of at least 60×22. No admin rights needed.
On Linux, `ping`, `ip` & `getent` are required; install `iw` to see the network name.

## Usage

```sh
whylag                     # ping 1.1.1.1
whylag --target 8.8.8.8    # ping another address
whylag --log pings.csv     # save every sample to a new CSV file
```

| Key | Action |
| --- | --- |
| `q` / `Ctrl+C` | Quit & show the summary |
| `p` | Pause the screen (checks keep running) |
| `t` | Switch between internet & router chart |
| `c` | Reset stats |
