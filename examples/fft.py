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
    hop_s = 256  // downsample # hop size

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

            last_time = frames2tick(total_frames)

        total_frames += read
        if read < hop_s:
            break

    print("Finished reading file:", filename)
    filename = filename.replace(".wav", ".mid")
    midi.save(filename)
    print("Saved MIDI file:", filename)
    play_midi(filename)

gen_notes(WAV_GUITAR)

NOTES = """Cb0	11
C0	12
Db0	13
C#0	13
D0	14
Eb0	15
D#0	15
Fb0	16
E0	16
F0	17
E#0	17
Gb0	18
F#0	18
G0	19
Ab0	20
G#0	20
A0	21
Bb0	22
A#0	22
Cb1	23
B0	23
C1	24
B#0	24
Db1	25
C#1	25
D1	26
Eb1	27
D#1	27
Fb1	28
E1	28
F1	29
E#1	29
Gb1	30
F#1	30
G1	31
Ab1	32
G#1	32
A1	33
Bb1	34
A#1	34
Cb2	35
B1	35
C2	36
B#1	36
Db2	37
C#2	37
D2	38
Eb2	39
D#2	39
Fb2	40
E2	40
F2	41
E#2	41
Gb2	42
F#2	42
G2	43
Ab2	44
G#2	44
A2	45
Bb2	46
A#2	46
Cb3	47
B2	47
C3	48
B#2	48
Db3	49
C#3	49
D3	50
Eb3	51
D#3	51
Fb3	52
E3	52
F3	53
E#3	53
Gb3	54
F#3	54
G3	55
Ab3	56
G#3	56
A3	57
Bb3	58
A#3	58
Cb4	59
B3	59
C4	60
B#3	60
Db4	61
C#4	61
D4	62
Eb4	63
D#4	63
Fb4	64
E4	64
F4	65
E#4	65
Gb4	66
F#4	66
G4	67
Ab4	68
G#4	68
A4	69
Bb4	70
A#4	70
Cb5	71
B4	71
C5	72
B#4	72
Db5	73
C#5	73
D5	74
Eb5	75
D#5	75
Fb5	76
E5	76
F5	77
E#5	77
Gb5	78
F#5	78
G5	79
Ab5	80
G#5	80
A5	81
Bb5	82
A#5	82
Cb6	83
B5	83
C6	84
B#5	84
Db6	85
C#6	85
D6	86
Eb6	87
D#6	87
Fb6	88
E6	88
F6	89
E#6	89
Gb6	90
F#6	90
G6	91
Ab6	92
G#6	92
A6	93
Bb6	94
A#6	94
Cb7	95
B6	95
C7	96
B#6	96
Db7	97
C#7	97
D7	98
Eb7	99
D#7	99
Fb7	100
E7	100
F7	101
E#7	101
Gb7	102
F#7	102
G7	103
Ab7	104
G#7	104
A7	105
Bb7	106
A#7	106
Cb8	107
B7	107
C8	108
B#7	108
Db8	109
C#8	109
D8	110
Eb8	111
D#8	111
Fb8	112
E8	112
F8	113
E#8	113
Gb8	114
F#8	114
G8	115
Ab8	116
G#8	116
A8	117
Bb8	118
A#8	118
Cb9	119
B8	119
C9	120
B#8	120
Db9	121
C#9	121
D9	122
Eb9	123
D#9	123
Fb9	124
E9	124
F9	125
E#9	125
Gb9	126
F#9	126
G9	127
Ab9	128
G#9	128
A9	129
Bb9	130
A#9	130
B9	131
B#9	132""".split("\n")
