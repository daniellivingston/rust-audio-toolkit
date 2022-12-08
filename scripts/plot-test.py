import numpy as np
import scipy as sp
import matplotlib.pyplot as plt
import pandas as pd
import librosa as lr
from librosa.display import specshow
from pysndfx import AudioEffectsChain

# Load audio with LibROSA.
y, sr = lr.load('../bin/c3-major-scale-piano.wav', sr=None)

def pitch_hps(audio_samples,
              sample_rate=sr,
              window_length=4096,
              hop_length=1024,
              window=np.hanning,
              partials=5,
              plot=False):
    """Estimate the pitch contour in a monophonic audio signal."""

    f0s = []
    frequencies = np.fft.rfftfreq(window_length, 1 / sample_rate)
    window = window(window_length)
    pad = lambda a: np.pad(a,
                           (0, window_length - len(a)),
                           mode='constant',
                           constant_values=0)

    # Low cut filter audio at 50 Hz.
    audio_samples = AudioEffectsChain().highpass(50)(audio_samples)

    # Go through audio frame-by-frame.
    for i in range(0, len(audio_samples), hop_length):

        # Fourier transform audio frame.

        frame = window * pad(audio_samples[i:window_length + i])
        spectrum = np.fft.rfft(frame)

        # Downsample spectrum.
        spectra = []
        for n in range(1, partials + 1):
            s = sp.signal.resample(spectrum, len(spectrum) // n)
            spectra.append(s)

        # Truncate to most downsampled spectrum.
        l = min(len(s) for s in spectra)
        a = np.zeros((len(spectra), l), dtype=spectrum.dtype)
        for i, s in enumerate(spectra):
            a[i] += s[:l]

        # Multiply spectra per frequency bin.
        hps = np.product(np.abs(a), axis=0)

        # TODO Blur spectrum to remove noise and high-frequency content.
        #kernel = sp.signal.gaussian(9, 1)
        #hps = sp.signal.fftconvolve(hps, kernel, mode='same')

        # TODO Detect peaks with a continuous wavelet transform for polyphonic signals.
        #peaks = sp.signal.find_peaks_cwt(np.abs(hps), np.arange(1, 3))

        # Pick largest peak, it's likely f0.
        peak = np.argmax(hps)
        f0 = frequencies[peak]
        f0s.append(f0)

        if plot:

            # Plot partial magnitudes individually.
            for s, ax in zip(spectra,
                             plt.subplots(len(spectra), sharex=True)[1]):
                ax.plot(np.abs(s))
            plt.suptitle('Partials')
            plt.show()

            # Plot combined spectra.
            plt.imshow(np.log(np.abs(a)), aspect='auto')
            plt.title('Spectra')
            plt.colorbar()
            plt.show()

            # Plot HPS peak.
            plt.plot(np.arange(len(hps)), np.abs(hps))
            plt.scatter(peak, np.abs(hps[peak]), color='r')
            plt.title('HPS peak')
            plt.show()
            return

    f0s = np.array(f0s)

    # Median filter out noise.
    f0s = sp.signal.medfilt(f0s, [21])

    return f0s

pitch_hps(y, plot=True)

specshow(lr.logamplitude(np.abs(lr.cqt(y, hop_length=1024, sr=sr))**2))
notes = lr.hz_to_midi(f0s + np.finfo(float).eps).round()
plt.step(np.arange(len(f0s)),
         notes - lr.note_to_midi('C1'),
         marker='|',
         label='Melody')
plt.title('CQT spectrogram with melody overlay')
plt.legend()
plt.show()
