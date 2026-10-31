# DB roles and migrations

Date: 2026-10-31

## Options

**Roles scripted once, migrations in backend, applied by CI/CD (chosen)**
- Roles: `db/roles.sql` creates `plaza_app` (idempotent). Prod: run once per environment by an operator, real password. Local: mounted in `/docker-entrypoint-initdb.d`, runs on first init.
- Migrations: `backend/migrations`. Schema evolves with the queries and `#[sqlx::test]` that depend on it.
- Applied by `db/migrate.sh` (sqlx-cli) as `plaza_owner`, never by the API.
  - Prod: `.github/workflows/deploy.yml`, `migrate` job before `deploy`, credentials from the `production` environment secret.
  - Local: the `migrate` service in `docker-compose.yml`.
- API only knows `DATABASE_URL` (`plaza_app`).

**Migrations on app start (rejected)**
- Owner credentials in the API's runtime environment.
- A failing migration crash-loops the app instead of stopping the deploy.
- Runs on every start of every replica.

**Roles in migrations (rejected)**
- Roles are cluster-level; `#[sqlx::test]` databases share them, so `CREATE ROLE` needs guards.
- Migrations would need superuser or `CREATEROLE`, and passwords end up in git and migration history.

**Migrations in `db/` (rejected)**
- Splits a schema change from the code using it.
- Needs path config for sqlx-cli and `#[sqlx::test]`.

## Consequences

- Least privilege: a compromised API can't alter schema or bypass RLS.
- A failed migration blocks the deploy before new code ships.
- New code ships after its migration, so migrations must stay compatible with the running version (expand/contract).
- sqlx-cli needed locally and in CI; major version must match the `sqlx` crate.
- New environments need `db/roles.sql` run before the first migration.
