BEGIN;

-- ---- users (password: "password") ----

INSERT INTO users (email, id, is_admin, password_hash) VALUES
    ('admin@plaza.test',  '10000000-0000-0000-0000-000000000004', true,  '$argon2id$v=19$m=19456,t=2,p=1$1sB0QNeu8p/jNe0jabkpqw$CDbafmNyozbuck9iMs76MAlrMYD+PrHh4UC8snU04Go'),
    ('alice@acme.test',   '10000000-0000-0000-0000-000000000001', false, '$argon2id$v=19$m=19456,t=2,p=1$1sB0QNeu8p/jNe0jabkpqw$CDbafmNyozbuck9iMs76MAlrMYD+PrHh4UC8snU04Go'),
    ('bob@acme.test',     '10000000-0000-0000-0000-000000000002', false, '$argon2id$v=19$m=19456,t=2,p=1$1sB0QNeu8p/jNe0jabkpqw$CDbafmNyozbuck9iMs76MAlrMYD+PrHh4UC8snU04Go'),
    ('carol@globex.test', '10000000-0000-0000-0000-000000000003', false, '$argon2id$v=19$m=19456,t=2,p=1$1sB0QNeu8p/jNe0jabkpqw$CDbafmNyozbuck9iMs76MAlrMYD+PrHh4UC8snU04Go')
ON CONFLICT DO NOTHING;

-- ---- teams ----

INSERT INTO teams (id, name) VALUES
    ('20000000-0000-0000-0000-000000000001', 'Acme Corp'),
    ('20000000-0000-0000-0000-000000000002', 'Globex')
ON CONFLICT DO NOTHING;

INSERT INTO team_members (role, team_id, user_id) VALUES
    ('owner',  '20000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001'),
    ('member', '20000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000002'),
    ('owner',  '20000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000003')
ON CONFLICT DO NOTHING;

-- ---- boards ----

INSERT INTO boards (id, name, team_id) VALUES
    ('30000000-0000-0000-0000-000000000001', 'Website Launch', '20000000-0000-0000-0000-000000000001'),
    ('30000000-0000-0000-0000-000000000002', 'Q4 Roadmap',     '20000000-0000-0000-0000-000000000002')
ON CONFLICT DO NOTHING;

-- ---- todos ----

INSERT INTO todos (assignee_id, board_id, description, id, status, team_id, title) VALUES
    ('10000000-0000-0000-0000-000000000001', '30000000-0000-0000-0000-000000000001', 'Ship v1 to prod',        '40000000-0000-0000-0000-000000000001', 'todo',        '20000000-0000-0000-0000-000000000001', 'Deploy to production'),
    ('10000000-0000-0000-0000-000000000002', '30000000-0000-0000-0000-000000000001', '',                       '40000000-0000-0000-0000-000000000002', 'in-progress', '20000000-0000-0000-0000-000000000001', 'Write landing copy'),
    (NULL,                                   '30000000-0000-0000-0000-000000000001', '',                       '40000000-0000-0000-0000-000000000003', 'done',        '20000000-0000-0000-0000-000000000001', 'Buy domain'),
    ('10000000-0000-0000-0000-000000000003', '30000000-0000-0000-0000-000000000002', 'Must never leak to Acme', '40000000-0000-0000-0000-000000000004', 'todo',        '20000000-0000-0000-0000-000000000002', 'Globex secret plan')
ON CONFLICT DO NOTHING;

COMMIT;
