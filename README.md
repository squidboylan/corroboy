# Corroboy

Copyright (c) 2018 Caleb Boylan

Corroboy is a basic Nintendo™ Game Boy™ emulator written in Rust.

This project started as a way to learn the Rust language. The goal of
corroboy is to be a modular emulator that is accurate enough to play games.
Once the emulator is accurate enough to play games without introducing any
bugs, I would like to add the features necessary to make it possible to have an
AI play games on the emulator.

I intend for users to only play games that are legally acquired, such as
Open Source or free ROMs.

The following is a checklist of corroboy features:
 - [x] CPU
   - [x] Instructions
   - [x] Interrupts
 - [x] GPU
   - [x] Background
   - [x] Sprites - implemented but buggy
   - [x] Window
 - [x] Joypad
 - [x] Timer
 - [x] MMU
 - [x] Cartridge support
   - [x] No MBC
   - [x] MBC1
   - [x] MBC2
   - [x] MBC3
     - [x] Timer
   - [x] MBC5
 - [ ] Audio

## Build Instructions

### Supported Platforms

The only platform Corroboy supports is Linux. MacOS and
Windows may currently work (it's untested) but even if they do I make no
promises that they will continue to work.

### Requirements

Because corroboy is a Rust project all of the standard tools to build rust
projects are required, specifically cargo and rustc. These can be installed
using your system packages or rustup, which is available at: https://rustup.rs/
. The only other required package/libary is the SDL2 dev package. On ubuntu
this can be installed using `apt-get install libsdl2-dev`.

### Building

In order to build the project run `cargo build --release`, this will build the
project with optimizations. The final binary will be available at
`target/release/corroboy`.

## Testing

Corroboy unit tests can be run using `cargo test`, Travis CI also runs these
same tests when a commit is pushed.

## Controls

The emulator controls are as follows:
  - Arrow keys -> D-Pad
  - Z -> A
  - X -> B
  - Enter -> Start
  - Right Shift -> Select

## License

Corroboy is licensed under the MIT license. See the `LICENSE` file in the project
for a copy of the license.

## Contact

Issues: For reporting bugs or missing features, please use the github issue
tracker on this project

Email: calebboylan AT gmail DOT com

# Sources

Below are sources I have found useful for understanding the GameBoy™ hardare
 - http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
 - http://bgb.bircd.org/pandocs.htm
