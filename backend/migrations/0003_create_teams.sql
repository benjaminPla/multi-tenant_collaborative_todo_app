CREATE TABLE teams (
    created_at timestamptz NOT NULL DEFAULT now(),
    id         uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    name       text        NOT NULL CHECK (length(name) BETWEEN 1 AND 100)
);

GRANT DELETE, INSERT, SELECT, UPDATE ON teams TO plaza_app;
