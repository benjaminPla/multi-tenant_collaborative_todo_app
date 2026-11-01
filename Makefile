# local dev only; docker-compose.yml runs the full stack

SHELL := bash
.SILENT:

BACKEND_PORT := 3000
COMPOSE      := docker compose --progress quiet
PSQL         := $(COMPOSE) exec -T db psql -U plaza_owner -d plaza -v ON_ERROR_STOP=1 -q

.PHONY: be-up db-migrate db-roles db-seed db-up fe-up

# ── backend ──────────────────────────────────────────────────────────────
be-up:
	cd backend && \
	DATABASE_URL=postgres://plaza_app:plaza_app@localhost:5432/plaza \
	DB_ACQUIRE_TIMEOUT_SECS=5 \
	DB_IDLE_TIMEOUT_SECS=600 \
	DB_MAX_CONNECTIONS=10 \
	DB_MIN_CONNECTIONS=1 \
	HOST=127.0.0.1 \
	HTTP_BODY_LIMIT_BYTES=65536 \
	HTTP_BODY_TIMEOUT_SECS=10 \
	HTTP_CONCURRENCY_LIMIT=32 \
	HTTP_TIMEOUT_SECS=10 \
	PORT=$(BACKEND_PORT) \
	RUST_LOG=info \
	SESSION_TTL_SECS=86400 \
	SSE_MAX_CONNECTIONS=256 \
	cargo watch -q -x run

# ── db ───────────────────────────────────────────────────────────────────
db-migrate:
	$(COMPOSE) run --rm migrate

db-roles:
	$(PSQL) < db/roles.sql

db-seed:
	$(PSQL) < db/seed.sql
	echo "login: admin@plaza.test / password"

db-up:
	$(COMPOSE) up -d --wait db

# ── frontend ─────────────────────────────────────────────────────────────
fe-up:
	cd frontend && BACKEND_URL=http://localhost:$(BACKEND_PORT) npm run dev
