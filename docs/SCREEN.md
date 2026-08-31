# Screen

* What whylag shows while it runs.
* How it works: [ARCHITECTURE.md](ARCHITECTURE.md). Every command & key: [COMMANDS.md](COMMANDS.md).

## The Screen

```
+------------------------------------------+
| whylag   HomeWifi  ▂▄▆_  -67 dBm         |   header
+------------------------------------------+
| Internet ping (last 60s)   [t] swap      |
|                     █                    |   bar chart
|              ▃      █       ▅            |
| ▂▂▃▂▂▃▂▂▂▂▃▅▇▃▂▂▂▂▂█▂▂▂▂▃▂▂▇▂▂           |
+------------------------------------------+
| Router 4ms │ Net 38ms │ Loss 2% │ Drops 1  |   stats
| STATUS: Wifi OK, internet laggy          |   status
+------------------------------------------+
| 12:04 Signal dropped to 1 bar            |
| 12:05 Lost connection                    |   event log
| 12:05 Back online after 8s               |
+------------------------------------------+
| q quit  p pause  t swap  c clear         |   keys
+------------------------------------------+
```

## Top To Bottom

| Part | Shows |
| --- | --- |
| **Header** | Network name, signal bars & signal number. |
| **Bar chart** | The main feature. See [The Bar Chart](ARCHITECTURE.md#the-bar-chart). |
| **Stats** | Quick numbers. |
| **Status** | One sentence saying what is wrong. |
| **Event log** | The last 5 things that happened. |
| **Keys** | What each key does. |
