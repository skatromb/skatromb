FROM python:latest

RUN apt update --quiet && \
    apt install --quiet --yes --no-install-recommends \
        default-jdk
WORKDIR /usr/app/

COPY requirements.txt .
RUN pip install --upgrade pip && \
    pip install -r requirements.txt

# install MySQL jdbc https://dev.mysql.com/downloads/connector/j/
ARG mysql_jdbc=mysql-connector-j_8.0.31-1ubuntu20.04_all.deb
RUN wget "https://dev.mysql.com/get/Downloads/Connector-J/$mysql_jdbc" && \
    dpkg --install $mysql_jdbc && \
    rm $mysql_jdbc

COPY . .
