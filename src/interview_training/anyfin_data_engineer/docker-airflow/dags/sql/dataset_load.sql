INSERT INTO dataset
    (WITH app_cycle AS
              (SELECT applications.id         AS application_id,
                      applications.customer_id,
                      applications.created_at AS app_created_at,
                      cycles.created_at       AS cycle_created_at,
                      cycles.status           AS cycle_status,
                      cycles.dpd
               FROM cycles
                        INNER JOIN applications ON applications.customer_id = cycles.customer_id
                   AND cycles.created_at < applications.created_at)

     SELECT app_loan.application_id,
            app_loan.created_at,
            app_loan.customer_id,
            app_loan.applications_before,
            COALESCE(app_loan.loans_before, 0)         AS loans_before,
            COALESCE(cycle_paid.paid_cycles_before, 0) AS paid_cycles_before,
            COALESCE(cycle_30.avg_dpd, 0)              AS avg_dpd_30_days_before,
            COALESCE(cycle_30.max_dpd, 0)              AS max_dpd_30_days_before,
            COALESCE(cycle_60.avg_dpd, 0)              AS avg_dpd_60_days_before,
            COALESCE(cycle_60.max_dpd, 0)              AS max_dpd_60_days_before
     FROM
         -- application and loan aggregates
         (SELECT applications.id                                                                         AS application_id,
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

     WHERE app_loan.application_id IS NOT NULL
);
