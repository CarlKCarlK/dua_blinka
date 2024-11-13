# blinky

This repo is the code used for the Embedded Rust Hardware Debug Probe workshop taught at the
Seattle Rust User Group in November 2024.

## Prerequisites
1. Install [Rust](https://rust-lang.org/tools/install).
2. Add dependencies:
   ### Linux:
   # Debian-based
       `sudo apt install -y pkg-config libudev-dev cmake git`
   # RPM-based
       `dnf install libusbx-devel libudev-devel cmake git`

   ### macOS:
  * No platform-specific requirements.  Proceed to `All Platforms`, below.

   ### Windows:
  * Install CMake and add to `$PATH`.

   ### All platforms:
   i. Install `probe-rs`:
   `cargo install probe-rs --locked`

3. Set up Hardware:  Set up as per [schematic diagram](https://app.cirkitdesigner.com/project/c8efdf17-e924-4550-8c7a-da5c56bd626e)

4. Teach `rustc` how to compile for the Raspberry Pi Pico's RP2040 processor:
   ```
   rustup install target thumbv6m-none-eabi
   ```

5. Test Your Setup:
   After `git clone`ing this repo, `cd` into the crate root folder and type `cargo run` to verify your
   board is set up correctly.

If the LED is flashing, your board is set up.

Pressing the button will change the LED flashing mode in the following sequence:
LED flashing modes:
```mermaid
flowchart LR
FastFlash --> SlowFlash --> On --> Off --> FastFlash
```

Long-pressing the button always resets to the first (FastFlash) LED state.

6. Begin debugging:
   `git checkout bugs` to switch to a buggy version of this repo.
   `cargo run` will compile and flash the buggy code.  The LED will no longer flash.  Now it is up to
   you to troubleshoot, find and fix the bug(s).

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
