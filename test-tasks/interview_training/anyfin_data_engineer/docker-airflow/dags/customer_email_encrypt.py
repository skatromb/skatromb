from datetime import datetime

from airflow import DAG
from airflow.operators.postgres_operator import PostgresOperator

default_args = {"owner": "airflow"}

with DAG(
    dag_id="encrypt_customer_email",
    start_date=datetime(2021, 3, 30),
    schedule_interval="@daily",
    default_args=default_args,
    catchup=False,
) as dag:
    clean_table = PostgresOperator(
        task_id="clean_email",
        postgres_conn_id="postgres_default",
        sql="sql/customer_email_clean.sql",
    )

    load_encrypted_email = PostgresOperator(
        task_id="encrypt_email",
        postgres_conn_id="postgres_default",
        sql="sql/customer_email_encrypt.sql",
        params={"supersecurekey": "supersecurekey"},
    )

    clean_table >> load_encrypted_email
