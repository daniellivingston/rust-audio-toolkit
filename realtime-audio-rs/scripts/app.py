# Run this app with `python app.py` and
# visit http://127.0.0.1:8050/ in your web browser.

# example: https://dash.gallery/dash-time-gating/

from dash import Dash, html, dcc
from dash.dependencies import Input, Output, State
import plotly.express as px
import numpy as np
from collections import namedtuple
from scipy.io import wavfile

Waveform = namedtuple("Waveform", "time amplitude")

from scipy.fftpack import fft

class Audio():
    @classmethod
    def hanning_window(cls, window_size: int, hop_size: int) -> np.ndarray:
        return np.hanning(window_size)

    def __init__(self, filename: str = None):
        self._sample_rate: float = None
        self._sample_duration: float = None
        self._waveform: Waveform = None

        if filename is not None:
            self.load(filename)

    def load(self, filename) -> None:
        self._sample_rate, data = wavfile.read(filename)
        data = data.mean(axis=1)
        self._sample_duration = float(len(data)) / self._sample_rate

        self._waveform = Waveform(
            time=np.arange(0., self._sample_duration, 1. / self._sample_rate),
            amplitude=data,
        )

    def duration(self) -> float:
        return self._sample_duration
    def sample_rate(self) -> float:
        return self._sample_rate
    def waveform(self) -> Waveform:
        return self._waveform

def discrete_fourier_transform(audio: Audio) -> np.ndarray:
    waveform = audio.waveform()
    v_freq = abs(fft(abs(waveform.amplitude)))
    return (
        v_freq,#[:len(waveform.amplitude)//2]
        np.arange(0, audio.sample_rate() / 2., audio.duration())
    )

def plot_discrete_fourier_transform(audio: Audio):
    wf = audio.waveform()

    fft = discrete_fourier_transform(
        waveform=wf,
        sample_rate=audio.sample_rate()
    )
    fft = fft / np.linalg.norm(fft) # normalize

    stride = 1
    fig = px.scatter(
        x=freq[::stride],
        y=fft[::stride],
        labels=dict(x="Frequency (Hz)", y="Amplitude (Normalized)"),
        log_y=True
    )

    return html.Div(children=[
        html.Div("Frequency Domain"),
        dcc.Graph(id='graph-freq-domain', figure=fig)
    ])

def plot_time_series(audio: Audio):
    wf = audio.waveform()
    stride = 5 # save memory usage

    fig = px.scatter(
        x=wf.time[::stride],
        y=wf.amplitude[::stride],
        labels=dict(x="Time (s)", y="Amplitude"),
    )

    return html.Div(children=[
        html.Div("Time Domain"),
        dcc.Graph(id='graph-time-domain', figure=fig)
    ])

audio = Audio(filename='../bin/c3-major-scale-piano.wav')

app = Dash(__name__)
app.layout = html.Div(children=[
    html.H1('Frequency Analysis'),
    plot_time_series(audio),
    plot_discrete_fourier_transform(audio),
])

if __name__ == '__main__':
    app.run_server(debug=True)
