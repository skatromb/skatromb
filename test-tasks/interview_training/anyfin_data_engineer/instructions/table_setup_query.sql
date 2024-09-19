CREATE TABLE applications (
    id text,
    created_at timestamp,
    status text,
    customer_id integer,
    loan_id integer,
	email text
);

CREATE TABLE cycles (
	id text,
	created_at timestamp,
	loan_id integer,
	start_date date,
	end_date date,
	status text,
	dpd integer,
	due_date date,
	balance numeric,
	customer_id integer
);


CREATE TABLE loans (
	id integer,
	balance numeric,
	customer_id integer,
	status text
);
