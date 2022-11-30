# Run this app with `python app.py` and
# visit http://127.0.0.1:8050/ in your web browser.

# example: https://dash.gallery/dash-time-gating/

from dash import Dash, html, dcc
from dash.dependencies import Input, Output, State
import plotly.express as px
import pandas as pd

app = Dash(__name__)
app.layout = html.Div([
    dcc.Tabs(id="tabs", value="tab-1", children=[
        dcc.Tab(label='Tab one', value='tab-1'),
        dcc.Tab(label='Tab two', value='tab-2')
    ]),
    html.Div(id='tabs-content')
])

colors = {
    'background': '#111111',
    'text': '#7FDBFF'
}

def page1():
    df = pd.DataFrame({
        "Fruit": ["Apples", "Oranges", "Bananas", "Apples", "Oranges", "Bananas"],
        "Amount": [4, 1, 2, 2, 4, 5],
        "City": ["SF", "SF", "SF", "Montreal", "Montreal", "Montreal"]
    })

    fig = px.bar(df, x="Fruit", y="Amount", color="City", barmode="group")

    return html.Div(children=[
        html.H1(children='Hello Dash'),

        html.Div(children='''
            Dash: A web application framework for your data.
        ''',
            style = {
                "textAlign": "center",
                "color": colors['text']
            }
        ),

        dcc.Graph(
            id='example-graph',
            figure=fig
        )
    ])


@app.callback(Output('my-store', 'data'),
              Input('my-store-input', 'value'))
def update_store(value):
    return value

@app.callback(Output('current-store', 'children'),
              Input('my-store', 'modified_timestamp'),
              State('my-store', 'data'))
def display_store_info(timestamp, data):
    return f"The store currently contains {data} and the modified timestamp is {timestamp}"

def page2():
    return html.Div([
        dcc.Store(id='my-store'),
        dcc.RadioItems(['NYC', 'MTL', 'SF'], 'NYC', id='my-store-input'),
        html.Div(id='current-store')
    ])

@app.callback(Output('tabs-content', 'children'),
              Input('tabs', 'value'))
def render_content(tab):
    if tab == 'tab-1':
        return page1()
    else:
        return page2()

if __name__ == '__main__':
    app.run_server(debug=True)

