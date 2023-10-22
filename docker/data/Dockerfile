FROM python:3.12

RUN apt-get update --quiet && \
    apt-get install --quiet --yes --no-install-recommends --no-cache \
        default-jdk && \
    apt-get clean
WORKDIR /usr/app/

COPY requirements.txt .
RUN pip install --upgrade --no-cache-dir -r requirements.txt

# install MySQL jdbc https://dev.mysql.com/downloads/connector/j/
ARG mysql_jdbc=mysql-connector-j_8.0.31-1ubuntu20.04_all.deb
RUN wget --quiet "https://dev.mysql.com/get/Downloads/Connector-J/$mysql_jdbc" && \
    dpkg --install $mysql_jdbc && \
    rm $mysql_jdbc

COPY . .
