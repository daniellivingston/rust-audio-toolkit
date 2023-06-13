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
from aubio import miditofreq
from numpy import arange

sys.path.append(str(Path(__file__).parent))
from rat_notes import midi_note_to_str

WAV_C_MAJOR = str(Path(__file__).parent / "data" / "C_major.wav")
WAV_GUITAR = str(Path(__file__).parent / "data" / "guitar_c4_scale.wav")
WAV_PIANO  = str(Path(__file__).parent / "data" / "c3-major-scale-piano.wav")

import math

def freq_to_note(freq):
    notes = ['A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#']

    if freq == 0.:
        return "", 0

    note_number = 12 * math.log2(freq / 440) + 49
    note_number = round(note_number)

    note = (note_number - 1 ) % len(notes)
    note = notes[note]

    octave = (note_number + 8 ) // len(notes)

    return note, octave

def gen_notes(filename: str, samplerate: int = None):
    downsample = 1

    if samplerate is None:
        samplerate = 44100 // downsample

    win_s = 512 // downsample # fft size
    hop_s = 256 // downsample # hop size

    #s = aubio.source(filename, samplerate, hop_s)
    s = aubio.source(filename, hop_size=hop_s)
    print(f"""{filename}
------------------------
URI: {s.uri}
Sample Rate: {s.samplerate}
Duration: {s.duration}
Channels: {s.channels}
    """)
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
    _freqs = []
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
            _freqs.append(miditofreq(new_note[0]))

            last_time = frames2tick(total_frames)

        total_frames += read
        if read < hop_s:
            break

    print("Finished reading file:", filename)
    # filename = filename.replace(".wav", ".mid")
    # midi.save(filename)
    # print("Saved MIDI file:", filename)
    # play_midi(filename)
    return _freqs

def gen_freqs(filename: str, min_confidence = 0.3):
    downsample = 1

    win_s = 512 // downsample # fft size
    hop_s = 256 // downsample # hop size

    with aubio.source(filename, hop_size=hop_s) as source:
        print(f"""{filename}
    ------------------------
    URI: {source.uri}
    Sample Rate: {source.samplerate}
    Duration: {source.duration}
    Channels: {source.channels}
        """)
        samplerate = source.samplerate

        # --- Begin Analysis -------------------------------------- #
        pitch_o = aubio.pitch("default",
                              buf_size=win_s,
                              hop_size=hop_s,
                              samplerate=samplerate)

        notes_o = aubio.notes("default",
                              buf_size=win_s,
                              hop_size=hop_s,
                              samplerate=samplerate)

        # --- Read samples from file ------------------------------------- #
        _return = {
            "metadata": {
                "name": Path(source.uri).stem,
                "samplerate": samplerate,
                "num_samples": 0,
                "buf_size": win_s,
                "hop_size": hop_s,
                "channels": source.channels,
                "duration": source.duration
            },
            "pitch": [],
            "notes": []
        }

        i = 0
        total_frames = 0
        while True:
            samples, read = source()
            total_frames += read
            if read < hop_s:
                break

            time = i * hop_s

            pitch = pitch_o(samples)
            confidence = pitch_o.get_confidence()

            if (confidence > min_confidence):
                _return["pitch"].append(
                    ( time, pitch[0] )
                )

            note = notes_o(samples)

            if (note[0] != 0): # valid note found
                _return["notes"].append(
                    ( time, miditofreq(note[0]) )
                )

            i += 1

    return _return

def plot_data(data: dict):
    from matplotlib import pyplot as plt
    import numpy as np

    ax = plt.axes()

    pitch = np.array(data["pitch"])
    notes = np.array(data["notes"])

    print(f"Data shape: {pitch.shape=}; {notes.shape=}")
    
    ax.semilogy(pitch[:,0], pitch[:,1], '.')
    ax.semilogy(notes[:,0], notes[:,1], '.')

    for note in notes:
        t, n = note
        ax.annotate(freq_to_note(n), (t, n))

    #ax.set_xlim(0, data["metadata"]["duration"])
    ax.set_xlabel("Time")
    ax.set_ylabel("Frequency [Hz]")
    ax.grid(True)

    ax.set_title(data["metadata"]["name"])
    ax.set_title("channels: {}\nduration: {}\nsamplerate: {}".format(
            data["metadata"]["channels"],
            data["metadata"]["duration"],
            data["metadata"]["samplerate"]
        ), loc='left')
    ax.set_title("samples: {}\nhop size: {}\nbuff size: {}".format(
            data["metadata"]["num_samples"],
            data["metadata"]["hop_size"],
            data["metadata"]["buf_size"]
        ), loc='right')

    plt.show()


data = gen_freqs(WAV_C_MAJOR)
plot_data(data)

