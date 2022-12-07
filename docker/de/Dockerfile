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
ENV MYSQL_JDBC_PATH=/usr/share/java/mysql-connector-j-8.0.31.jar

CMD pyspark \
    --driver-class-path ${MYSQL_JDBC_PATH} --jars ${MYSQL_JDBC_PATH} \
    --conf "spark.executor.memory=10g" \
    --conf "spark.driver.memory=4g"

COPY . .
