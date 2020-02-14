FROM ubuntu:19.10
WORKDIR app
COPY config .
RUN apt-get update && \
    apt-get install python3.8 -y && \
    apt-get install python3-pip -y && \
    apt-get clean && \
    \
    update-alternatives --install /bin/python python /bin/python3 0
    update-alternatives --install /bin/pip3 pip /bin/pip 0
    pip install -r requirements.txt

# docker run -it python
