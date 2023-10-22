FROM godatadriven/pyspark:3.4

WORKDIR /app
COPY requirements.txt /app

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean && \
    pip install --upgrade --no-cache-dir -r requirements.txt
