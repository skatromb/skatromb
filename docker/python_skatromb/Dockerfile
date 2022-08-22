FROM python:latest

ENV SPARK_HOME=/usr/local/lib/python3.10/site-packages/pyspark/

RUN apt update --quiet && \
    apt install --quiet --yes --no-install-recommends \
        default-jdk
WORKDIR /usr/app/
COPY * .

RUN python -m pip install --upgrade pip && \
    pip install -r requirements.txt

# install MySQL jdbc
WORKDIR $SPARK_HOME
ARG mysql_jdbc=mysql-connector-java_8.0.27-1debian11_all.deb
RUN wget "https://dev.mysql.com/get/Downloads/Connector-J/$mysql_jdbc" && \
    dpkg --install $mysql_jdbc && \
    rm $mysql_jdbc
ENV MYSQL_JDBC_PATH=/usr/share/java/mysql-connector-java-8.0.27.jar
RUN pyspark --driver-class-path $MYSQL_JDBC_PATH --jars $MYSQL_JDBC_PATH
