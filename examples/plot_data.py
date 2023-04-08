import numpy as np
from matplotlib import pyplot as plt
import subprocess
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent

AUDIO_FILE = f"{SCRIPT_DIR}/data/c3-major-scale-piano.wav"

p = subprocess.run(["aubiopitch", "-i", AUDIO_FILE, "--pitch-unit", "Hz"], capture_output=True, check=True)

# TODO: simplify this
arr = np.array([list(map(float, line.split(" "))) for line in p.stdout.decode("utf-8").splitlines()])

print("time, frequency")
print(arr)

with open(f"{SCRIPT_DIR}/data/notes.txt", "r") as f:
    note_names = []
    note_freqs = []

    for line in f.readlines():
        name, freq = line.split(", ")
        note_names.append(name)
        note_freqs.append(float(freq))

# ----------------------------------------------------- #

min_freq = np.min(arr[:, 1])
max_freq = np.max(arr[:, 1])

fig, axes = plt.subplots(nrows=1, ncols=1)

fig.suptitle(Path(AUDIO_FILE).stem)

for (name, freq) in zip(note_names, note_freqs):
    axes.annotate(name, xy=(0.0, freq))
    axes.axhline(freq, color="red", linestyle="--")

axes.scatter(arr[:, 0], arr[:, 1])
axes.set_ybound(min_freq, max_freq)
plt.show()
