CREATE TABLE users (
    created_at    timestamptz NOT NULL DEFAULT now(),
    email         text        NOT NULL UNIQUE,
    id            uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    is_admin      boolean     NOT NULL DEFAULT false,
    password_hash text        NOT NULL
);

ALTER TABLE users
    ADD CONSTRAINT users_email_format    CHECK (email ~ '^[^@[:space:]]+@[^@[:space:]]*\.[^@[:space:]]*$'),
    ADD CONSTRAINT users_email_length    CHECK (octet_length(email) <= 254),
    ADD CONSTRAINT users_email_lowercase CHECK (email = lower(email));

GRANT DELETE, INSERT, SELECT, UPDATE ON users TO plaza_app;
