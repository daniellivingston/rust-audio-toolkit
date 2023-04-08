import numpy as np
from matplotlib import pyplot as plt
import subprocess
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent

AUDIO_FILE = f"{SCRIPT_DIR}/data/c3-major-scale-piano.wav"

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

print("time, frequency")
print(freqs)

# ----------------------------------------------------- #

min_freq = np.min(freqs[:, 1])
max_freq = np.max(freqs[:, 1])

fig, axes = plt.subplots(nrows=1, ncols=1)

fig.suptitle(Path(AUDIO_FILE).stem)

# for beat in get_beats():
#     axes.axvline(beat, color="grey", linestyle="--")

for onset in get_onsets():
    axes.axvline(onset, color="green", linestyle="--")

for (name, freq) in zip(note_names, note_freqs):
    axes.annotate(name, xy=(0.0, freq))
    axes.axhline(freq, color="red", linestyle="--")

axes.scatter(freqs[:, 0], freqs[:, 1])
axes.set_ybound(min_freq, max_freq)
plt.show()
