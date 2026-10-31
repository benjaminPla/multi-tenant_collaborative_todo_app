CREATE TYPE platform_role AS ENUM ('admin', 'user');

CREATE TABLE users (
    created_at    timestamptz   NOT NULL DEFAULT now(),
    email         text          NOT NULL UNIQUE CHECK (email = lower(email) AND length(email) <= 254),
    id            uuid          PRIMARY KEY DEFAULT gen_random_uuid(),
    password_hash text          NOT NULL,
    role          platform_role NOT NULL DEFAULT 'user'
);

GRANT INSERT, SELECT, UPDATE ON users TO plaza_app;
