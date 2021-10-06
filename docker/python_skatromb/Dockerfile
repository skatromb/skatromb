FROM python:latest

RUN apt update --quiet && \
    apt install --quiet --yes --no-install-recommends \
      default-jdk
WORKDIR /config
COPY config .

# Ставим зависимости
RUN pip install -r requirements.txt
