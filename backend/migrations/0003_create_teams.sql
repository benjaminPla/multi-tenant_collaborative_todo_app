CREATE TABLE teams (
    created_at timestamptz NOT NULL DEFAULT now(),
    id         uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    name       text        NOT NULL
);

ALTER TABLE teams
    ADD CONSTRAINT teams_name_length  CHECK (length(name) BETWEEN 1 AND 100),
    ADD CONSTRAINT teams_name_trimmed CHECK (name !~ '^[[:space:]]|[[:space:]]$');

GRANT DELETE, INSERT, SELECT, UPDATE ON teams TO plaza_app;
