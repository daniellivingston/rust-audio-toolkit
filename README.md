# RTA

#### *Real-time Audio: Cross-platform Rust dynamic audio project*

[![Build Status](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml)

This project is an attempt to build a high-performance, high-quality music practicing tool.

## Usage

### PSARC Parsing

#### Print summary of PSARC file

```sh
./target/debug/rta read --summary ../../bin/psarc/dlc/karmapolice_m.psarc
# -or- #
cargo run -- read --summary ./bin/psarc/dlc/karmapolice_m.psarc
```

### Audio Devices

### Realtime Device Experiment

```sh
./target/debug/rta device
# -or- #
cargo run -- device
```

#### List available audio devices

```sh
./target/debug/rta device --list
# -or- #
cargo run -- device --list
```

## Compiling

You will need to [download the Rust compiler and toolchain](https://www.rust-lang.org/tools/install).

The entire project is compiled with:

```sh
cargo build
```

Binaries will be under the generated `target/` folder.

```sh
$ cargo build
  Finished dev [unoptimized + debuginfo] target(s) in 0.05s

$ cd target/debug/
$ ./rta --help
```

## Release Milestones

- **v1.0.0: Public release**
  - Audio format parsing:
    - PSARC (PlayStation Archive; for Rocksmith 2014 support)
    - GuitarPro 5 (`.gp5`)
  - Audio device in & out:
    - Support low-latency audio pipeline: microphone -> FFT -> guitarFX -> output speaker
    - Implement music feature extraction using FFT
    - Implement plug-in system for varying guitar / amp / vocal / etc. live effects
  - 3D Graphics Frontend
    - Support for audio-synced 3D rendering of musical notes
  - (todo...fill in more)

### Progress Tracker

- [ ] **v0.2:**
  - [ ] Implementation of music feature extraction algorithm
  - [ ] Fully-featured PSARC file parsing
  - [x] Implement initial pass at music feature extraction
  - [ ] Deconstruct initial `manifest.ini` file from PSARC
- [x] **v0.2.0: Skeleton file parsing & audio analysis**
  - [x] Unified CLI client interface
  - [x] Primitive logging support
  - [x] Supports audio input capture & audio output playback
  - [x] PSARC header & TOC table parsing
- [x] **v0.1.0: Initial version**
