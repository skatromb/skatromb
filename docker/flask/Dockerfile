FROM python:3.12
COPY requirements.txt .
RUN pip install -r requirements.txt --no-cache-dir
COPY app.py .
CMD flask --debug --app app.py run --port 8000
