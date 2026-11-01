# plaza

Multi-tenant collaborative todo board. Teams own boards, boards own todos, and everyone on a board sees changes live.

Rust (axum, sqlx) · PostgreSQL 18 · React + TypeScript (no UI libraries) · nginx · Docker Compose · GitHub Actions

## Run

```sh
docker compose up --build    # http://localhost:8080
docker compose down -v       # reset: wipes the database
```

Postgres starts, `db/roles.sql` runs on first init, then one-shot `migrate` and `seed` containers, then the backend and nginx serving the frontend.

| Login (password `password`) | Role |
|---|---|
| `admin@plaza.test` | admin, every team |
| `alice@acme.test` | owner of Acme Corp |
| `bob@acme.test` | member of Acme Corp |
| `carol@globex.test` | owner of Globex |

To see real-time updates, open "Website Launch" as alice and bob in two browsers (one private window) and move a card.

### Local development

Needs Docker, Rust with `cargo-watch`, and Node 24.

```sh
make db-up db-migrate db-seed   # postgres in docker
make be-up                      # backend on :3000, cargo watch
make fe-up                      # frontend on :5173, vite proxies /api to :3000
```

### Tests

```sh
cd backend
cargo test                                                                       # unit
DATABASE_URL=postgres://plaza_owner:plaza_owner@localhost:5432/plaza \
  cargo test -- --ignored                                                        # integration, needs make db-up db-roles
```

Integration tests (`#[ignore]`) run each case in a fresh migrated and seeded database via `#[sqlx::test]`, connecting as `plaza_app` so RLS applies. CI runs clippy, unit and integration tests on every pull request.

## Architecture

```
browser ─► nginx ─/api─► axum
                         ├─ tower stack: panic → 500, load shedding, timeouts, body limits, tracing
                         ├─ PEP: session → policy registry → resource loader → PIP → PDP
                         ├─ handler → use case → repository ─► Postgres (RLS)
                         └─ todo events ─► broadcast bus ─► SSE streams
```

The backend is hexagonal: `domain` (entities, value objects, ports, pure policy), `application` (use cases), `infrastructure` (Postgres adapters, one file per query), `interface` (HTTP). The frontend is plain HTML elements, one global stylesheet of design tokens, and colocated component styles.

### Multi-tenancy

A team is a tenant. One shared schema, `team_id` on every team-owned row, and Postgres row-level security on `boards` and `todos`: each query runs in a transaction that sets `app.team_id` locally, so a forgotten filter returns no rows instead of another team's data. The API connects as `plaza_app` (`NOBYPASSRLS`); migrations run as `plaza_owner`. Non-members get 404, so team ids can't be probed.

### Auth

Server-side sessions: a 256-bit random token in an `HttpOnly`, `Secure`, `SameSite=Lax` cookie, stored as its SHA-256 hash. Passwords are argon2id with OWASP parameters; unknown emails verify against a dummy hash so timing doesn't reveal accounts. A password change revokes the user's sessions.

Authorization is a PEP/PDP pipeline at the route level: every route is registered as public or guarded by an action and a resource loader, and anything unregistered is denied. The PDP is a pure function, unit-tested per role.

### Real-time

Server-Sent Events, one stream per open board: `GET /api/teams/{id}/boards/{board_id}/todos/events`.

- Create, update and delete publish an event (actor plus todo) after the database commit, through an in-memory `tokio::sync::broadcast` behind a domain port.
- The board view opens an `EventSource` on mount and closes it on unmount. It applies events by `version`, so stale events and its own echoes are no-ops, refetches after a reconnect, and toasts other people's changes.
- Streams send keep-alives every 15 s and close after 5 minutes, so reconnects go back through the PEP and pick up logouts and membership changes. A lagging client gets `resync`; open streams are capped; shutdown closes them gracefully.

Concurrent edits use optimistic locking: `PATCH` sends the `version` it read, and a stale one gets 409. The frontend moves cards optimistically and, on 409, reapplies the change once if the other edit touched different fields; otherwise the server version wins and the user is told.

### Decisions

[ADRs](docs/adr): [tenancy](docs/adr/01-tenancy.md) · [runtime](docs/adr/02-runtime.md) · [db roles and migrations](docs/adr/03-db-roles-and-migrations.md) · [authorization](docs/adr/04-authorization.md) · [frontend](docs/adr/05-frontend.md)

All limits and settings are environment variables, declared in `docker-compose.yml` and the `Makefile`. Logs are JSON in Docker (`LOG_FORMAT`); a 500 logs its cause, while the client only sees `internal error`.

## Trade-offs and room to improve

- No signup endpoint: users are created by admins from the control panel (Users view, `POST /api/users`).
- Single backend process: the event bus is in memory. Scaling out needs Postgres `LISTEN/NOTIFY` or Redis behind the same port.
- Missed events are recovered by refetching on reconnect, not replayed (`Last-Event-ID`).
- No pagination on list endpoints.
- No spans in logs beyond the per-request one.
- Expired sessions are only deleted when presented again; abandoned rows stay.
- Drag and drop has no keyboard alternative, and cards have no order within a column.
- No client router, so views have no URLs.
- Tests cover the domain and the Postgres adapters; there are no API-level or browser end-to-end tests.
- CI applies migrations before tests run, so migrations must stay backward compatible (expand/contract).
