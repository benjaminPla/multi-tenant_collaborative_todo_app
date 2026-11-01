CREATE TABLE users (
    created_at    timestamptz NOT NULL DEFAULT now(),
    email         text        NOT NULL UNIQUE CHECK (email = lower(email) AND length(email) <= 254),
    id            uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    is_admin      boolean     NOT NULL DEFAULT false,
    password_hash text        NOT NULL
);

GRANT DELETE, INSERT, SELECT, UPDATE ON users TO plaza_app;
