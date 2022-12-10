import logging
from datetime import datetime
from os import environ
from zoneinfo import ZoneInfo

from pyspark.sql.session import SparkSession

logging.basicConfig(
    format="%(asctime)s %(levelname)s %(filename)s %(lineno)s: %(message)s",
    level=logging.INFO,
)
logger = logging.getLogger(__name__)

tz = ZoneInfo("CET")
logger.warning(f"Job started at")

db_url = environ["DB_URL"]
db_schema = environ["DB_SCHEMA"]
db_table = environ["DB_TABLE"]
db_indexed_column = environ["DB_INDEXED_COLUMN"]
db_user = environ["DB_USER"]
db_password = environ["DB_PASSWORD"]

spark = SparkSession.builder.getOrCreate()

min_indexed_col, max_indexed_col = (
    spark.read.format("jdbc")
    .option("url", f"jdbc:mysql://{db_url}/{db_schema}")
    .option("user", db_user)
    .option("password", db_password)
    .option(
        "query",
        "\n".join(
            [
                "SELECT",
                f"    MIN({db_indexed_column}) AS lower_bound,",
                f"    MAX({db_indexed_column}) AS upper_bound",
                f"FROM {db_table}",
            ]
        ),
    )
    .load()
    .collect()[0]
)

logger.info(f"MIN value in {db_indexed_column} = {min_indexed_col}")
logger.info(f"MAX value in {db_indexed_column} = {max_indexed_col}")

started_at = datetime.now(tz)
df = (
    spark.read.format("jdbc")
    .option("url", f"jdbc:mysql://{db_url}/{db_schema}")
    .option("user", db_user)
    .option("password", db_password)
    .option("dbtable", f"{db_schema}.{db_table}")
    # partitioning try
    .option("partitionColumn", db_indexed_column)
    .option("lowerBound", min_indexed_col)
    .option("upperBound", max_indexed_col)
    .option("numPartitions", 1_000)
    .load()
)

(
    df.write.parquet(
        f"/Users/skatromb/code/skatromb/outputs/spark/to_parquet_{db_table}",
        mode="overwrite",
    )
)

spark.stop()  # Just to have logging more close to tail

ended_at = datetime.now(tz)
logger.info(f"Job ended")

logger.info(f"TIME PASSED: {ended_at - started_at} for `{db_table}`")
