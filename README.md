# Realtime Audio / GFX [RS]

[![Build Status](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/daniellivingston/realtime-audio-rs/actions/workflows/rust.yml)

This project is an attempt to build a high-performance, high-quality music practicing tool.

## Release Milestones

- [ ] v0.1.0:
  - [ ] v0.1.0: Initial skeleton project
  - [ ] v0.1.1: Supports audio input capture & audio output playback
- [ ] v1.0.0: Initial release

## Sub-projects

During early stages of development, core functional areas are split into isolated crates to enable faster
research and development.

Below, these crates are described. Visit their respective README for more information.

| Crate                   | Description                                       |
|-------------------------|---------------------------------------------------|
| `realtime-audio-rs`     | Exploratory R&D into audio device input & output. |
| `afx-song-file-toolkit` | File parsing for common music & media formats.    |
