"""Try Flask for markdown."""

from flask import Flask

app = Flask(__name__)
app.config.from_prefixed_env()


@app.route('/')
def root():
    """Root endpoint."""
    return '<h1>Hello, World!</h1>'
