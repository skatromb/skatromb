import csv
from datetime import datetime
from os import environ
from zoneinfo import ZoneInfo

import sqlalchemy

tz = ZoneInfo("CET")
db_user = environ["DB_USER"]
db_password = environ["DB_PASSWORD"]
db_host = environ["DB_URL"]
db_schema = environ["DB_SCHEMA"]
db_table = environ["DB_TABLE"]
output_file = (
    f"/Users/skatromb/code/skatromb/outputs/sql_alchemy/to_csv_{db_table}.csv"
)

engine = sqlalchemy.create_engine(
    f"mysql+pymysql://{db_user}:{db_password}@{db_host}/{db_schema}", echo=True
)

with engine.connect() as connection:
    # query metadata
    metadata = sqlalchemy.MetaData()
    table = sqlalchemy.Table(
        f"{db_table}", metadata, autoload=True, autoload_with=engine
    )
    query = sqlalchemy.select([table])
    # execute query
    i = 1
    cursor = connection.execute(query)
    print(f"cursor execution done {datetime.now(tz)}")
    # go writing
    with open(output_file, "w", newline="") as csv_file:
        csv_writer = csv.writer(csv_file)
        while True:
            rows = cursor.fetchmany(10_000)
            print("fetched 10_000 rows")
            if not rows:
                break
            csv_writer.writerows(rows)
            i += 1
        cursor.close()
