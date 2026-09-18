# Commands

* Every way to start whylag & every key it listens to.
* How it works: [ARCHITECTURE.md](ARCHITECTURE.md). What the screen looks like: [SCREEN.md](SCREEN.md).

## Start

```sh
whylag
```

## Options

| Command | Does |
| --- | --- |
| `whylag --target 8.8.8.8` | Ping a different address. |
| `whylag --log pings.csv` | Save every ping to a file. |

* You can mix them: `whylag --target 8.8.8.8 --log pings.csv`

## Keys

| Key | Does |
| --- | --- |
| `q` or `Ctrl+C` | Quit & show the summary. |
| `p` | Pause the screen. |
| `t` | Swap internet & router bars. |
| `c` | Clear stats. |

## Notes

* `--target` takes an IPv4 or IPv6 address.
* `--help` shows options. `--version` shows the version.
* Pause holds the screen still. Checks & logging keep running.
* Clear starts new stats. It does not erase the CSV file.
* CSV columns: `unix_seconds,target,router,router_ms,internet_ms`.
* A blank ping value means no reply. Times use Unix seconds.
* The log path must not exist. Each sample is flushed to disk.
