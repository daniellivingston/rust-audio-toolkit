import numpy as np
import scipy.fft as fft
import scipy.io.wavfile as wavfile
from matplotlib import pyplot as plt
plt.style.use('ggplot')

class Notes(object):
    NOTES_JSON = "notes.json"

    def __init__(self):
        import json
        with open(Notes.NOTES_JSON, 'r') as f:
            self.json = json.loads(f.read())
        self._notes = set(self.json.items())

    @property
    def names(self):
        return {x[0] for x in self._notes}

    @property
    def frequencies(self):
        return np.array([x[1] for x in self._notes])

    def __repr__(self):
        return f"Notes[len={len(self.json)}]\n\t" + \
            f"{self.names=}\n\t" + \
            f"{self.frequencies=}"

class Audio(object):
    def __init__(self, wav_filename: str):
        self.filename = wav_filename
        self.sample_rate, self.data = wavfile.read(wav_filename)

    def __repr__(self) -> str:
        return f"Audio({self.filename=},\n\t{self.sample_rate=},\n\t{self.data=})"

    @property
    def num_samples(self):
        return self.get_data(keep_channels=False).shape[0]

    @property
    def duration(self) -> float:
        return np.floor(self.num_samples / self.sample_rate)

    @property
    def sample_times(self):
        return np.linspace(0., self.duration, self.num_samples)

    def get_data(self, keep_channels=False, normalized=False, abs=False) -> np.ndarray:
        rvalue = self.data
        if not keep_channels:
            rvalue = rvalue.mean(axis=1)
        if normalized:
            rvalue = rvalue / rvalue.max()
        if abs:
            rvalue = np.abs(rvalue)
        return rvalue

    def fft(self):
        data = self.get_data(
            keep_channels=True,
            normalized=True,
            abs=True
        )
        assert(len(data) == self.num_samples)

        delta = self.sample_times[1] - self.sample_times[0]

        print("=" * 50)
        print(f"Sample rate: {self.sample_rate} Hz")
        print(f"Duration: {self.duration} seconds")
        print(f"# of samples: {self.num_samples}")
        print(f"Delta: {delta} seconds")
        print("=" * 50)

        # fourier transform and frequency domain
        # https://makersportal.com/blog/2018/9/13/audio-processing-in-python-part-i-sampling-and-the-fast-fourier-transform
        N = self.num_samples # total points in signal
        Y_k = np.fft.fft(data)[0:int(N/2)]/N # FFT function from numpy
        Y_k[1:] = 2*Y_k[1:] # need to take the single-sided spectrum only
        Pxx = np.abs(Y_k) # be sure to get rid of imaginary part
        f = self.sample_rate * np.arange((N/2))/N; # frequency vector
        return (f, Pxx)

        # transform = np.abs(fft.fft(data))
        # freqs = fft.fftfreq(self.num_samples, delta)

        # return (transform, freqs)

    def plot(self):
        fig, ax = plt.subplots(2)
        fig.suptitle("Frequency analysis: '???.wav'")

        ax[0].plot(
            self.sample_times,
            self.get_data(normalized=True),
            'g',
            label="Freq. v. Time",
            linewidth=0.5
        )
        ax[0].set_xlabel("Time (s)")
        ax[0].set_ylabel("Amplitude")

        f, Pxx = self.fft()
        ax[1].plot(f, Pxx)
        ax[1].set_xscale('log')
        ax[1].set_yscale('log')
        ax[1].set_xlabel('Frequency (Hz)')
        ax[1].set_ylabel('Amplitude')

        for freq in Notes().frequencies:
            ax[1].axvline(freq,
                          color='grey',
                          linestyle='--',
                          linewidth=0.3,
                          label="Note")

        # continue here...
        # https://makersportal.com/blog/2018/9/17/audio-processing-in-python-part-ii-exploring-windowing-sound-pressure-levels-and-a-weighting-using-an-iphone-x

        plt.savefig("fft.pdf")

def main():
    audio = Audio("../bin/c3-major-scale-piano.wav")
    audio.plot()

if __name__ == '__main__':
    main()
