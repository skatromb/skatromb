FROM ubuntu:19.10
WORKDIR config
COPY config .
RUN apt-get update && \
    apt-get install python3.8 -y && \
    apt-get install python3-pip -y && \
    apt-get clean && \
    \
    pip3 install -r requirements.txt

# docker run -it python
