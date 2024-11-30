# dua_blinka

* Two virtual (and real) LEDs, working in parallel.
* If you hold the button, the LEDs send SOS.
* Each LED follows a schedule of on/off times. A schedule is a no_alloc Vec of Durations.
* If you hold the button long enough, the LEDs will react before you
   release.

This project is based on <https://github.com/U007D/blinky_probe/tree/main> from the
Embedded Rust Hardware Debug Probe workshop taught at the
Seattle Rust User Group in November 2024.

## State Diagram

```mermaid
stateDiagram-v2
    FastAlternating --> FastTogether : Tap
    FastAlternating --> SOS : Hold
    FastTogether --> Slow : Tap
    FastTogether --> SOS : Hold
    Slow --> AlwaysOn : Tap
    Slow --> SOS : Hold
    AlwaysOn --> AlwaysOff : Tap
    AlwaysOn --> SOS : Hold
    AlwaysOff --> FastAlternating : Tap
    AlwaysOff --> SOS : Hold
    SOS --> FastAlternating : Tap
```

## Wiring Diagram

[![Wiring Diagram](wiring_diagram.png)](https://app.cirkitdesigner.com/project/38f41aba-e97e-46a3-81b6-35f196153c90)

## Video

[![Watch the video](https://img.youtube.com/vi/_iQKyh3FGX4/0.jpg)](https://youtu.be/_iQKyh3FGX4)

## License

Licensed under either:

* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.
