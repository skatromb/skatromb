[comment]: <> (TODO)

## 0. Setup environment 
   
1. I've noticed, that [applications.csv](SQL-data/applications.csv) has two problems:
* It contains redundant columns at the end filled of `;;;;;;;;;;;;`.
* It contains only 5k fulfilled rows, others are an empty dataset `;;;;;;;;;;;;;;;;;`.

Don't know if it's a bug or feature of test task, but think it is important to inform you. 
I think they meant to be deleted.
I worked with them as they are intended to exist (although useless).

2. While joining tables I've noticed another problem:
`cycles.loan_id` matches `loan.id` only partially.
Example of `cycles.loan_id`: 649154725. 
   
Here's the proof:
 ```postgresql
 SELECT COUNT(*)
 FROM cycles
 WHERE loan_id IS NOT NULL;
 -- 49886 rows
 
 SELECT COUNT(*)
 FROM cycles
     INNER JOIN loans ON cycles.loan_id = loans.id
 ;
 -- 1956 rows
 
 SELECT COUNT(*)
 FROM loans
 WHERE id IS NULL;
 -- 0 rows
 ```

## 1. Build ER diagram

_tip_: In a diagram, I describe relations based on data, not on DDL statements.

[ER diagram](er_diagram.jpeg)

## 2. Basic queries

### A. `customer_id` of customers which have `dpd` > 10 days

``` postgresql
SELECT DISTINCT customer_id 
FROM cycles
WHERE dpd > 10;
```

### B. `application_id` for the first application of each customer

```postgresql
SELECT 
    id as first_application_id,
    customer_id
FROM 
    (
    SELECT id,
        ROW_NUMBER() OVER (PARTITION BY customer_id ORDER BY created_at) AS history_num,
        customer_id
    FROM applications
    ) AS subq

WHERE history_num = 1
    AND customer_id IS NOT NULL;
```

### C. `customer_id` of customers that had more than 1 application within a time period of 30 days

```postgresql
SELECT DISTINCT app.customer_id
FROM applications AS app
WHERE EXISTS
    (
    SELECT *
    FROM applications AS app2
    WHERE app2.customer_id = app.customer_id
        AND app.id != app2.id
        AND app.created_at BETWEEN app2.created_at - INTERVAL '30 day'
                               AND app2.created_at + INTERVAL '30 day'
    );
```
### D. `customers_ids` who had only open or overdue cycles

```postgresql
SELECT customer_id
FROM cycles
WHERE customer_id NOT IN
    (SELECT DISTINCT customer_id
    FROM cycles
    WHERE status = 'paid');
```

### E. ordered list of customers over their percentage of overdue cycles

```postgresql
SELECT customer_id,
       (COUNT(status) FILTER (WHERE status = 'overdue'))::NUMERIC /
        COUNT(*) AS all_count
FROM cycles
GROUP BY 1
ORDER BY 2 DESC;
```

## 3. Datasets

I don't like the code's length, in production I will think about decomposing it to be more modular and readable.
But it works, I hope.

Also, I've found that in task description was duplicate column of `number of paid cycles`. So I dropped the duplicate 
```postgresql
WITH app_cycle AS
    (SELECT applications.id AS application_id,
            applications.customer_id,
            applications.created_at AS app_created_at,
            cycles.created_at AS cycle_created_at,
            cycles.status AS cycle_status,
            cycles.dpd
    FROM cycles
        INNER JOIN applications ON applications.customer_id = cycles.customer_id
            AND cycles.created_at < applications.created_at)

SELECT app_loan.application_id,
       app_loan.created_at,
       app_loan.customer_id,
       app_loan.applications_before,
       COALESCE(app_loan.loans_before, 0)        AS loans_before,
       COALESCE(cycle_paid.paid_cycles_before, 0)     AS paid_cycles_before,
       COALESCE(cycle_30.avg_dpd, 0) AS avg_dpd_30_days_before,
       COALESCE(cycle_30.max_dpd, 0) AS max_dpd_30_days_before,
       COALESCE(cycle_60.avg_dpd, 0) AS avg_dpd_60_days_before,
       COALESCE(cycle_60.max_dpd, 0) AS max_dpd_60_days_before
FROM
     -- application and loan aggregates
     (SELECT applications.id AS application_id,
             applications.loan_id,
             applications.created_at,
             applications.customer_id,
             ROW_NUMBER() OVER (PARTITION BY applications.customer_id ORDER BY applications.created_at) - 1
                             AS applications_before,
             ROW_NUMBER() OVER (PARTITION BY loans.customer_id ORDER BY applications.created_at) - 1 AS loans_before

      FROM applications
          INNER JOIN loans ON loans.id = applications.loan_id
     ) AS app_loan

    -- counting paid cycles before the application
    LEFT JOIN
     (SELECT application_id,
             customer_id,
             COUNT(*) AS paid_cycles_before

      FROM app_cycle
      WHERE cycle_status = 'paid'
      GROUP BY application_id, customer_id
    ) AS cycle_paid
        ON cycle_paid.application_id = app_loan.application_id

    -- counting 30 days before metrics
    LEFT JOIN
        (SELECT application_id,
                customer_id,
                AVG(dpd) AS avg_dpd,
                MAX(dpd) AS max_dpd
        FROM app_cycle
        WHERE cycle_created_at > app_created_at - INTERVAL '30 DAY'
        GROUP BY application_id, customer_id
        ) AS cycle_30
            ON cycle_30.application_id = app_loan.application_id

    -- counting 60 days before metrics
    LEFT JOIN
        (SELECT application_id,
                customer_id,
                AVG(dpd) AS avg_dpd,
                MAX(dpd) AS max_dpd
        FROM app_cycle
        WHERE cycle_created_at > app_created_at - INTERVAL '30 DAY'
        GROUP BY application_id, customer_id
        ) AS cycle_60
            ON cycle_60.application_id = app_loan.application_id

WHERE app_loan.application_id IS NOT NULL;
```

## 4. Load the dataset daily in DAG

I've created [daily DAG](docker-airflow/dags/dataset_load.py) for that purpose. 
It fills the [`dataset` table](instructions/dataset.ddl)

## 5. Encryption

I've [created a new table](instructions/encrypted_emails.ddl) with the encrypted customer's emails: `customer_email_encrypted`.

I've used a postgres extension `CREATE EXTENSION pgcrypto;` for encryption
and used the key `supersecurekey` which is passed as parameter from DAG

Emails encrypted by the function `pgp_sym_encrypt(email::bytea, supersecurekey)`. 
To decrypt the column you need to select column with reversing function 
`pgp_sym_decrypt(encrypted_email, supersecurekey)`:

[DAG](docker-airflow/dags/customer_email_encrypt.py) that inserts the encrypted data to the table

SQL script which can help to read the encrypted data:
```postgresql
SELECT PGP_SYM_DECRYPT(email::bytea, 'supersecurekey') FROM customer_email_encrypted;
```