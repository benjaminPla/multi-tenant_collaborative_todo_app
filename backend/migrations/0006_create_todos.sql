CREATE TYPE todo_status AS ENUM ('todo', 'in-progress', 'done');

CREATE TABLE todos (
    assignee_id uuid,
    board_id    uuid        NOT NULL,
    created_at  timestamptz NOT NULL DEFAULT now(),
    description text        NOT NULL DEFAULT '' CHECK (length(description) <= 5000),
    id          uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
    status      todo_status NOT NULL DEFAULT 'todo',
    team_id     uuid        NOT NULL,
    title       text        NOT NULL CHECK (length(title) BETWEEN 1 AND 200),
    updated_at  timestamptz NOT NULL DEFAULT now(),
    version     integer     NOT NULL DEFAULT 1 CHECK (version > 0),
    FOREIGN KEY (team_id, assignee_id) REFERENCES team_members (team_id, user_id) ON DELETE SET NULL (assignee_id),
    FOREIGN KEY (team_id, board_id) REFERENCES boards (team_id, id) ON DELETE CASCADE
);

CREATE INDEX todos_team_id_board_id_idx ON todos (team_id, board_id);

ALTER TABLE todos ENABLE ROW LEVEL SECURITY;
ALTER TABLE todos FORCE ROW LEVEL SECURITY;

CREATE POLICY admin_access ON todos
    AS PERMISSIVE FOR ALL TO plaza_app
    USING      ((SELECT current_setting('app.is_admin', true)) = 'true')
    WITH CHECK ((SELECT current_setting('app.is_admin', true)) = 'true');

CREATE POLICY team_isolation ON todos
    AS PERMISSIVE FOR ALL TO plaza_app
    USING      (team_id = (SELECT nullif(current_setting('app.team_id', true), '')::uuid))
    WITH CHECK (team_id = (SELECT nullif(current_setting('app.team_id', true), '')::uuid));

GRANT DELETE, INSERT, SELECT, UPDATE ON todos TO plaza_app;
