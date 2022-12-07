# from pyspark.sql.session import SparkSession as spark

from datetime import datetime
from os import environ
from zoneinfo import ZoneInfo

tz = ZoneInfo("CET")
print(started_at := datetime.now(tz))

# Profiles = 15m rows
df = (
    spark.read.format("jdbc")
    .option("url", environ["DB_URL"])
    .option("user", environ["DB_USER"])
    .option("password", environ["DB_PASSWORD"])
    .option("dbtable", f"{environ['DB_TABLE']}")
    # .option("fetchsize", f"{10_000}")
    # partitioning try
    # .option("partitionColumn", "ID")
    # .option("lowerBound", "0")
    # .option("upperBound", "20000000")
    # .option("numPartitions", 128)
    .load()
)

(df.write.parquet("/Users/skatromb/code/skatromb/jdbc_test", mode="overwrite"))

print(ended_at := datetime.now(tz))

print(f"TIME PASSED: {ended_at - started_at}")
