CREATE TABLE boards (
    created_at timestamptz NOT NULL DEFAULT now(),
    id         uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    name       text        NOT NULL CHECK (length(name) BETWEEN 1 AND 100),
    team_id    uuid        NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    UNIQUE (team_id, id)
);

ALTER TABLE boards ENABLE ROW LEVEL SECURITY;
ALTER TABLE boards FORCE ROW LEVEL SECURITY;

CREATE POLICY team_isolation ON boards
    AS PERMISSIVE FOR ALL TO plaza_app
    USING      (team_id = (SELECT current_setting('app.team_id', true)::uuid))
    WITH CHECK (team_id = (SELECT current_setting('app.team_id', true)::uuid));

GRANT DELETE, INSERT, SELECT, UPDATE ON boards TO plaza_app;
