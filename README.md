# dua_blinka

[![Rust Embedded](https://img.shields.io/badge/Rust-Embedded-blue?style=flat-square)](https://www.rust-lang.org/) [![Embassy](https://img.shields.io/badge/Framework-Embassy-orange?style=flat-square)](https://embassy.dev/)

A Rust-based embedded example featuring abstract and real LEDs controlled with `Embassy`

## Features

- **Two virtual (and real) LEDs** working in parallel.
- **SOS signal** when holding the button.
- **Scheduled LED behavior** using a `no_alloc` Vec of `Durations`.
- **Responsive input handling**—holding the button long enough triggers a reaction *before* release.

## Related Article

**[How Rust & Embassy Shine on Embedded Devices (Part 1): Insights for Everyone and Nine Rules for Embedded Programmers](https://medium.com/@carlmkadie/how-rust-embassy-shine-on-embedded-devices-part-1-9f4911c92007)**  
by **Carl M. Kadie & Brad Gibson**, free on Medium.

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

- MIT license (see LICENSE-MIT file)
- Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.
