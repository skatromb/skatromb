# Data Engineer hiring task - Anyfin

## Run and setup Airflow locally

```bash
git clone https://github.com/puckel/docker-airflow.git
```
```bash
cd docker-airflow
```
#### Copy the file ```docker-compose-anyfin.yml``` you will find in this repository, into the ```docker-airflow``` repository
#### Then run:
```bash
docker-compose -f docker-compose-anyfin.yml up -d
```

#### At this point Airflow should be up and running ( [localhost:8080](localhost:8080) by default)
#### An empty local postgres database is also installed on your localhost, that can be used for tasks 4 and 5. Credentials are

```bash
host:       localhost
username:   username
password:   supersecure
schema:     postgres
port:       5432
```

#### A **.sql** file called [table_setup_query](table_setup_query.sql) is provided as the DDL code to create the three tables required (*cycles, applications, loans*). After creating them, you can load the respective CSV files that you have been provided with.

#### If you have problems setting up the task do not hesitate to contact us at data-engineering@anyfin.com
