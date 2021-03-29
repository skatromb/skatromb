[comment]: <> (TODO)

## 0. Setup environment 
   
1. I've noticed, that [applications.csv](SQL-data/applications.csv) has two problems:
* It contains redundant columns at the end filled of `;;;;;;;;;;;;`.
* Last 5k rows contains empty dataset `;;;;;;;;;;;;;;;;;`.

Don't know if it's a bug or feature of test task, but think it is important to inform you. 
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