from airflow import DAG
from airflow.operators.postgres_operator import PostgresOperator
from datetime import datetime


default_args = {"owner": "airflow"}

with DAG(
    dag_id="load_dataset",
    start_date=datetime(2021, 3, 30),
    schedule_interval="@daily",
    default_args=default_args,
    catchup=False,
) as dag:
    clean_dataset = PostgresOperator(
        task_id="clean_dataset",
        postgres_conn_id="postgres_default",
        sql="sql/dataset_clean.sql",
    )
    load_dataset = PostgresOperator(
        task_id="load_dataset",
        postgres_conn_id="postgres_default",
        sql="sql/dataset_load.sql",
    )

    clean_dataset >> load_dataset
