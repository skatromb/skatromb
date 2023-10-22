"""Try Flask for markdown."""

from flask import Flask

app = Flask(__name__)


@app.route('/')
def root():
    """Root endpoint."""
    return '<h1>Hello, World!</h1>'
