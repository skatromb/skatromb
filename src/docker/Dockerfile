FROM ubuntu:latest

WORKDIR app
COPY config /app

RUN apt-get update && \
    apt-get upgrade -y && \
    \
    apt-get install -y \
        python3 \
        python3-pip \
        python3-venv

RUN apt-get install build-essential -y && \
    apt-get clean && \
    \
    python3 -m venv env && \
    source env/bin/activate && \
    pip install -r requirements.txt && \

# docker run -it python
