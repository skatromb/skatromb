FROM godatadriven/pyspark:latest

WORKDIR app
COPY requirements.txt /app

RUN apt-get update && \
    apt-get upgrade -y && \
    pip install -U -r requirements.txt

# docker run -it python
