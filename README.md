# Realtime Audio / GFX [RS]

[![Build Status](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml)

This project is an attempt to build a high-performance, high-quality music practicing tool.

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
- [ ] **v0.1: Skeleton file parsing & audio analysis**
  - [ ] *v0.1.2:*
    - [ ] Implement initial pass at music feature extraction
    - [ ] Deconstruct initial `manifest.ini` file from PSARC
  - [x] *v0.1.1:*
    - [x] Supports audio input capture & audio output playback
    - [x] PSARC header & TOC table parsing
  - [x] *v0.1.0: Initial skeleton project*

## Sub-projects

During early stages of development, core functional areas are split into isolated crates to enable faster
research and development.

Below, these crates are described. Visit their respective README for more information.

| Crate                   | Description                                       |
|-------------------------|---------------------------------------------------|
| `realtime-audio-rs`     | Exploratory R&D into audio device input & output. |
| `afx-song-file-toolkit` | File parsing for common music & media formats.    |
