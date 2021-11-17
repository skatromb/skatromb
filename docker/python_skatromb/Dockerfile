FROM python:latest

RUN apt update --quiet && \
    apt install --quiet --yes --no-install-recommends \
        default-jdk
WORKDIR /usr/app/
COPY requirements.txt .

# Ставим зависимости
RUN pip install -r requirements.txt
