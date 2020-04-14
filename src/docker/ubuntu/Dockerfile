FROM ubuntu:19.10

ENV http_proxy "http://t2rm-fgproxy.corp.tele2.ru:8080/"
WORKDIR app
# COPY requirements.txt /app

RUN apt-get update && \
    apt-get upgrade -y && \
    \
    # Пакеты
    apt-get install -y \
        make \
        docker \
   && \
   apt-get clean


# docker run -it ubuntu
