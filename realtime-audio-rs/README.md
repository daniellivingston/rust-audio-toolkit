# Realtime Audio RS

This Crate is an R&D exercise with the following goals:

1. Cross-platform audio in / out across various devices (mic, input cable, MIDI, etc.)
2. Instrument-agnostic pipeline (support for any live recorded instrument, incl. vocals, + MIDI support)
3. Low-latency passthru from input audio device to output audio device
4. Low-latency effects (FX) post-processed onto input audio device (ex.: reverb, distortion, chorus, echo)
5. Realtime (low-latency) audio stream analysis of musical characteristics, such as notes & chords played
6. Translation layer from audio input -> MIDI

## Usage

Within this folder, simply run:

```sh
cargo run
```
