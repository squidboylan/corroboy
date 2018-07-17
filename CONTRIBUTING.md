# Contributing Guide

Copyright (c) 2018 Caleb Boylan

## Project Structure

Corroboy is organized logically into modules based on the functionality and
correspondence to the emulated hardware.

In `src` directory there is a `main.rs` file which does arg parsing and sets up
a window for graphics. In the same directory there is the `corroboy` directory
which contains all the source for emulating the hardware. These pieces are
generally placed in their own subdirectories and are glued together into a full
system in the `mod.rs` file. Modules may even be divided into smaller
submodules as is the case with the `mmu` module and it's submodule `cartridge`.
Unit tests are also included in each module where possible in the `tests.rs`
file.

## Issues

Bug reports are a great way to contribute to the project. When reporting a bug
please specify the observed behavior, platform, game, and how to reproduce. If
possible to provide screenshots please do.
