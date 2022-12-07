# from pyspark.sql.session import SparkSession as spark

from datetime import datetime
from os import environ
from zoneinfo import ZoneInfo

tz = ZoneInfo("CET")
print(started_at := datetime.now(tz))

db_url = environ["DB_URL"]
db_schema = environ["DB_SCHEMA"]
db_table = environ["DB_TABLE"]
db_user = environ["DB_USER"]
db_password = environ["DB_PASSWORD"]

# Profiles = 15m rows
df = (
    spark.read.format("jdbc")
    .option("url", f"jdbc:mysql://{db_url}/{db_schema}")
    .option("user", db_user)
    .option("password", db_password)
    .option("dbtable", f"{db_schema}.{db_table}")
    # partitioning try
    # .option("partitionColumn", "ID")
    # .option("lowerBound", "0")
    # .option("upperBound", "20000000")
    # .option("numPartitions", 128)
    .load()
)

(
    df.write.partitionBy().parquet(
        f"/Users/skatromb/code/skatromb/outputs/spark/to_parquet_{db_table}",
        mode="overwrite",
    )
)

print(ended_at := datetime.now(tz))

print(f"TIME PASSED: {ended_at - started_at}")
