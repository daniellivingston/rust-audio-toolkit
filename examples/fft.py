# pip install aubio
# pip install mido
# pip install python-rtmidi
# https://github.com/aubio/aubio/tree/master/python/demos

import sys
import time
import aubio
import mido
from mido import Message, MetaMessage, MidiFile, MidiTrack, second2tick, bpm2tempo
from pathlib import Path

sys.path.append(str(Path(__file__).parent))
from rat_notes import midi_note_to_str

WAV_GUITAR = str(Path(__file__).parent / "data" / "guitar_c4_scale.wav")
WAV_PIANO  = str(Path(__file__).parent / "data" / "c3-major-scale-piano.wav")

class Note:
    def __init__(self, t, n):
        self.t = t
        self.n = (n[0], n[1], n[2])

        self.note = n[0]
        self.velocity = n[1]
        self.__idk = n[2]

    def __str__(self):
        note_str = ' '.join(["%.2f" % i for i in self.n])
        return "%.6f" % self.t + f": {note_str}"

    def __repr__(self):
        return str(self)

def play_midi(filename: str):
    portname = None

    with mido.open_output(portname) as output:
        try:
            midifile = MidiFile(filename)
            t0 = time.time()
            for message in midifile.play():
                print(message)
                output.send(message)
            print('play time: {:.2f} s (expected {:.2f})'.format(
                time.time() - t0, midifile.length
            ))
        except KeyboardInterrupt:
            print()
            output.reset()

def gen_notes(filename: str, samplerate: int = None):
    downsample = 1

    if samplerate is None:
        samplerate = 44100 // downsample

    win_s = 512 // downsample # fft size
    hop_s = 256 // downsample # hop size

    s = aubio.source(filename, samplerate, hop_s)
    print(f"{s}")
    samplerate = s.samplerate

    tolerance = 0.8

    notes_o = aubio.notes("default", win_s, hop_s, samplerate)

    # --- Initialize MIDI Track -------------------------------------- #
    midi = MidiFile()
    track = MidiTrack()
    midi.tracks.append(track)

    ticks_per_beat = midi.ticks_per_beat # default: 480
    bpm = 120 # default midi tempo

    tempo = bpm2tempo(bpm)
    track.append(MetaMessage('set_tempo', tempo=tempo))
    track.append(MetaMessage('time_signature', numerator=4, denominator=4))

    def frames2tick(frames, samplerate=samplerate):
        sec = frames / float(samplerate)
        return int(second2tick(sec, ticks_per_beat, tempo))

    # --- Read samples from file ------------------------------------- #
    last_time = 0
    total_frames = 0 # total number of frames read
    while True:
        samples, read = s()
        new_note = notes_o(samples)

        if (new_note[0] != 0): # valid note found
            delta = frames2tick(total_frames) - last_time

            # Add 'note_off' to track, if applicable
            if new_note[2] > 0:
                msg = Message(
                    'note_off',
                    note=int(new_note[2]),
                    velocity=127,
                    time=delta
                )
                track.append(msg)

            # Add 'note_on' to track
            msg = Message(
                'note_on',
                note=int(new_note[0]),
                velocity=int(new_note[1]),
                time=delta
            )
            track.append(msg)

            print("%.2f : %s" % (delta, midi_note_to_str(new_note[0])))

            last_time = frames2tick(total_frames)

        total_frames += read
        if read < hop_s:
            break

    print("Finished reading file:", filename)
    filename = filename.replace(".wav", ".mid")
    midi.save(filename)
    print("Saved MIDI file:", filename)
    # play_midi(filename)

gen_notes(WAV_GUITAR)

