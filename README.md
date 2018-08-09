# Corroboy

Copyright (c) 2018 Caleb Boylan

Corroboy is a basic Nintendo™ Game Boy™ emulator written in Rust.

This project started as a way to learn the Rust language. The goal of
Corroboy is to be a modular emulator that is accurate enough to play games.
Currently the emulator works for some simple games, but many complicated games
do not work or are playable but have bugs. The emulator is also
unoptimized and should work on any modern laptop or better system, but won't
work on low power devices such as a raspberry pi.

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

## CS461 and CS410

For CS461 (Open Source development) and CS410 (Rust Programming) I worked on
implementing cartridge support and window support in Corroboy. The files corresponding to
cartridge support are `src/corroboy/mmu/cartridge/*`. The window support only
consisted of about ~100 lines of code in the `src/corroboy/gpu/background.rs`
file. Overall the project went fairly well. I wrote unit tests for each of my
cartridge types and did manual testing which makes me rather confident
that the code is correct. The window code is much harder to unit test as it
is graphics code so I did not implement unit tests, but I did manual testing and it
works for everything i've tested it with. I also did a bunch of bug fixing in
code that I wrote before the classes started, but those are spread out over the
project and the easiest way to see what I did there is to look at the commit
history.

## Sources

Below are sources I have found useful for understanding the GameBoy™ hardare
 - http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
 - http://bgb.bircd.org/pandocs.htm
