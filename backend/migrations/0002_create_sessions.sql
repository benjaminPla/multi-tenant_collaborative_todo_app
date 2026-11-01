CREATE TABLE sessions (
    created_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL,
    token_hash bytea       PRIMARY KEY,
    user_id    uuid        NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

ALTER TABLE sessions
    ADD CONSTRAINT sessions_token_hash_length CHECK (octet_length(token_hash) = 32);

CREATE INDEX sessions_expires_at_idx ON sessions (expires_at);
CREATE INDEX sessions_user_id_idx ON sessions (user_id);

GRANT DELETE, INSERT, SELECT ON sessions TO plaza_app;
