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

class Audio():
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

def plot_time_series():
    audio = Audio(filename='../bin/c3-major-scale-piano.wav')
    wf = audio.waveform()

    stride = 5 # save memory usage

    fig = px.scatter(
        x=wf.time[::stride],
        y=wf.amplitude[::stride],
        labels=dict(x="Time (s)", y="Amplitude"),
    )

    return html.Div(children=[
        html.Div("Time Domain"),
        dcc.Graph(id='graph-time-series', figure=fig)
    ])

app = Dash(__name__)

app.layout = html.Div(children=[
    html.H1('Frequency Analysis'),
    plot_time_series()
])

if __name__ == '__main__':
    app.run_server(debug=True)
