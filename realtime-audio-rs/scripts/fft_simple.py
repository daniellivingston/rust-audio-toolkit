import numpy as np
from matplotlib import pyplot as plt
import scipy.io.wavfile as wavfile
import scipy.fft as fft

class Notes(object):
    NOTES_JSON = "notes.json"

    def __init__(self):
        import json
        with open(Notes.NOTES_JSON, 'r') as f:
            self.json = json.loads(f.read())

    def __repr__(self):
        s = ["Note\tFrequency"]
        for (key, value) in self.json.items():
            s.append([key.strip().upper() + "\t{value}"])
        return '\n'.join(s)

class Audio(object):
    def __init__(self, wav_filename: str):
        self.filename = wav_filename
        self.fs_rate, self.signal_original = wavfile.read(wav_filename)
        self.total_time = int(np.floor(len(self.signal_original) / self.fs_rate))
        self.sample_range = np.arange(0, self.total_time, self.time_period)
        self.total_samples = len(self.sample_range)

    def __repr__(self) -> str:
        return f"Audio [ freq_sampling = {self.fs_rate}, time = {self.total_time} ]"

def main():
    notes = Notes()
    audio = Audio("../bin/c3-major-scale-piano.wav")

    print(notes)
    print()
    print(audio)

if __name__ == '__main__':
    main()

