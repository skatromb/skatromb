FROM python:latest

WORKDIR /config
COPY config .
ENV GOOGLE_APPLICATION_CREDENTIALS=/config/our-audio-320911-0a4714398d4f.json

# Ставим зависимости
RUN pip install -U -r requirements.txt
