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
