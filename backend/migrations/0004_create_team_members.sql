CREATE TYPE team_role AS ENUM ('member', 'owner');

CREATE TABLE team_members (
    created_at timestamptz NOT NULL DEFAULT now(),
    role       team_role   NOT NULL DEFAULT 'member',
    team_id    uuid        NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    user_id    uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, user_id)
);

CREATE INDEX team_members_user_id_idx ON team_members (user_id);

GRANT DELETE, INSERT, SELECT, UPDATE ON team_members TO plaza_app;
