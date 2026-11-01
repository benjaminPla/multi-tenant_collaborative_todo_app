# Tenancy

Date: 2026-10-30

## Options

**Shared schema + RLS (chosen)**
- `team_id` (team = tenant) on every team-owned row, enforced by Postgres RLS.
- Two roles: `plaza_owner` (owns tables, runs migrations) and `plaza_app` (API, `NOSUPERUSER NOBYPASSRLS`).
- `ENABLE` + `FORCE ROW LEVEL SECURITY` on team-owned tables.
- Policy: `team_id = (SELECT nullif(current_setting('app.team_id', true), '')::uuid)` for both `USING` and `WITH CHECK`. `nullif` because a pooled connection returns `''` once a transaction-local setting ends.
- Admins: second permissive policy on `app.is_admin`, see [04](04-authorization.md).
- Per request: `BEGIN` → `set_config('app.team_id', $1, true)` → queries → `COMMIT`. Transaction-scoped, pool-safe.
- Global tables have no RLS; accessed only by narrow queries filtered by `user_id`.

**Schema-per-team (rejected)**
- Migrations run once per team; new team = `CREATE SCHEMA` + migrate.
- `search_path` switching on pooled connections can leak a team into the next request.
- Catalog bloat and slow migrations at thousands of teams.
- Users belong to many teams, so `users` must live outside team schemas anyway (hybrid model).
- Breaks sqlx compile-time query checks (table names vary per request).

**Database-per-team (rejected)**
- Strongest isolation, but one connection pool and migration run per team.
- Fits few large or regulated tenants, not many small teams.

## Consequences

- Isolation enforced by the database, not only by app code. A forgotten `WHERE team_id = …` returns zero rows. Fails closed when no team is set.
- One migration set, new team = one `INSERT`.
- Logical isolation only; teams share tables, so per-team backup/restore is not trivial.
- Not covered by RLS: SQL injection (prevented by bound parameters), views without `security_invoker`, `SECURITY DEFINER` functions, `LISTEN/NOTIFY`, constraint checks (unique/FK see all rows).
