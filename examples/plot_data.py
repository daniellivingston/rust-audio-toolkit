import numpy as np
import sys
from matplotlib import pyplot as plt
import subprocess
from pathlib import Path
import mido
import random

SCRIPT_DIR = Path(__file__).parent

C_Major_Scale = {
    "file": f"{SCRIPT_DIR}/data/C_major.wav",
    "notes": ["C", "D", "E", "F", "G", "A", "B", "C", "B", "A", "G", "F", "E", "D", "C"],
}

AUDIO_FILE = C_Major_Scale["file"]

def get_notes():
    p = subprocess.run([
           "aubionotes",
           AUDIO_FILE
        ],
        capture_output=True,
        check=True
    )
    output = p.stdout.decode("utf-8").splitlines()
    return np.array([list(map(float, line.split())) for line in output[1:-1]])

def get_beats():
    p = subprocess.run([
           "aubiotrack",
           AUDIO_FILE
        ],
        capture_output=True,
        check=True
    )
    output = p.stdout.decode("utf-8")
    return np.array([float(line) for line in output.splitlines()])

def get_onsets():
    p = subprocess.run([
           "aubioonset",
           AUDIO_FILE
        ],
        capture_output=True,
        check=True
    )
    output = p.stdout.decode("utf-8")
    return np.array([float(line) for line in output.splitlines()])

def get_frequencies(method: str):
    p = subprocess.run([
           "aubiopitch",
            "-i", AUDIO_FILE,
            "--pitch-unit", "Hz",
            "-T", "seconds",
            "--pitch", method,
            "--pitch-tolerance", "0.4",
            "--silence", "-90",
            "--hopsize", "256",
            "--bufsize", "2048",
        ],
        capture_output=True,
        check=True
    )

    output = p.stdout.decode("utf-8")

    # TODO: simplify this
    arr = [list(map(float, line.split(" "))) for line in output.splitlines()]

    return np.array(arr)

PITCH_METHODS = [
    "default",
    "schmitt",
    "fcomb",
    "mcomb",
    "specacf",
    "yin",
    "yinfft",
    "yinfast"
]

with open(f"{SCRIPT_DIR}/data/notes.txt", "r") as f:
    note_names = []
    note_freqs = []

    for line in f.readlines():
        name, freq = line.split(", ")
        note_names.append(name)
        note_freqs.append(float(freq))

# ----------------------------------------------------- #

freqs = get_frequencies(PITCH_METHODS[0])
onsets = get_onsets()

print("time, frequency")
print(freqs)

print("onsets", onsets)

# ----------------------------------------------------- #

time = freqs[:,0]
idx = np.array(list(map(lambda x: np.argmin(np.abs(time - x)), onsets)))

x = np.zeros((len(onsets) - 1,))
captured_notes = []

for i in range(len(idx) - 1):
    x[i] = np.mean(freqs[idx[i]:idx[i+1], 1])
    captured_notes.append(note_names[np.argmin(np.abs(note_freqs - x[i]))])

print(x)
print(captured_notes)

# ----------------------------------------------------- #

min_freq = np.min(freqs[:, 1])
max_freq = np.max(freqs[:, 1])

fig, axes = plt.subplots(nrows=2, ncols=1, sharex=True)

fig.suptitle(Path(AUDIO_FILE).stem)

# --- FREQUENCY ------------------------------------------------------ #

# for beat in get_beats():
#     axes.axvline(beat, color="grey", linestyle="--")

for onset in onsets:
    axes[0].axvline(onset, color="green", linestyle="--")

for (name, freq) in zip(note_names, note_freqs):
    axes[0].annotate(name, xy=(0.0, freq))
    axes[0].axhline(freq, color="red", linestyle="--")

axes[0].scatter(freqs[:, 0], freqs[:, 1])
axes[0].set_ybound(min_freq, max_freq)

# --- NOTES ---------------------------------------------------------- #

def get_note_name(midi_note):
    # Calculate pitch value (number of semitones away from A4)
    pitch = midi_note - 69

    # Calculate octave and note name
    octave = pitch // 12
    note = pitch % 12
    note_names = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']
    note_name = note_names[note]

    # Return note name with octave number
    return f"{note_name}{octave}"

notes = get_notes()

axes[1].scatter(notes[:, 1], notes[:, 0])

for note in notes:
    name = get_note_name(int(note[0]))
    axes[1].annotate(name, xy=(note[1], note[0]))


plt.show()


# ----------------------------------------------------- #
