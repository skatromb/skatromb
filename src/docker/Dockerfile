FROM ubuntu:19:10

WORKDIR app
COPY config /app

RUN apt-get update && \
    apt-get upgrade -y && \
    \
    # Ставим питон и ессеншлзы
    apt-get install -y \
        python3-pip \
        python3.8 \
        build-essential \
    && \
    apt-get clean && \
    \
    # Делаем питону алиасы
    update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.8 0 && \
    update-alternatives --install /usr/bin/python python /usr/bin/python3.8 0 && \
    update-alternatives --install /usr/bin/pip pip /usr/bin/pip3 0 && \
    \
    # Ставим зависимости
    pip install -r requirements.txt

# docker run -it python
