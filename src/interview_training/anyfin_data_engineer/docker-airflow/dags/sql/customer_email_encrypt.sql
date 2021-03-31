INSERT INTO customer_email_encrypted
    (
        SELECT DISTINCT applications.customer_id,
                        pgp_sym_encrypt(applications.email, '{{ params.supersecurekey }}')
        FROM cycles
                 INNER JOIN applications ON applications.customer_id = cycles.customer_id
        WHERE cycles.dpd > 10
    );
