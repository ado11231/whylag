# Screen

* What whylag shows while it runs.
* How it works: [ARCHITECTURE.md](ARCHITECTURE.md). Every command & key: [COMMANDS.md](COMMANDS.md).

## The Screen

```
 whylag  HomeWifi  ▂▄__  -64 dBm            Gateway 192.168.1.1     header
────────────────────────────────────────────────────────────────
┌ Internet latency  1.1.1.1 ───────────────────────────────────┐
│                   ██                                         │  bar chart
│ ▃▃ ▅▅ ▆▆ ▇▇       ██ ▁▁ ▃▃ ▄▄ ▅▅ ▆▆          ▁▁ ▂▂ ▃▃ ▄▄ ▆▆ │
│ ██ ██ ██ ██ ▅▅ ▇▇ ██ ██ ██ ██ ██ ██ ▅▅ ▆▆ ▇▇ ██ ██ ██ ██ ██ ██ │
└─────────────────────────────────────────── Scale 0 to 300 ms ┘
┌ Details ─────────────────────────────────────────────────────┐
│ Latency  53 ms Average  49 ms Best     20 ms Worst    79 ms  │  details
│ Jitter   12 ms Loss     2.5%  Drops    0     Offline  00:00  │
│ Router   4 ms  DNS      OK    Low wifi 00:00 Session  05:12  │
└──────────────────────────────────────────────────────────────┘
 ● Connection healthy                                             status
 Events ─────────────────────────────────────────────────────────
 04:10  Wifi signal low                                           event log
 04:31  Connection lost
 04:39  Connection restored after 8s
 q Quit   p Pause   t Switch chart   c Reset                      keys
```

## Top To Bottom

| Part | Shows |
| --- | --- |
| **Header** | Network name, signal bars, signal number & gateway address. |
| **Bar chart** | The main feature. See [The Bar Chart](ARCHITECTURE.md#the-bar-chart). |
| **Details** | Latency, average, best, worst, jitter, loss, drops, offline time, router or internet ping, DNS, low wifi time & session time. |
| **Status** | One sentence saying what is wrong. |
| **Event log** | The last 5 things that happened. |
| **Keys** | What each key does. |
