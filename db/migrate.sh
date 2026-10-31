#!/usr/bin/env bash
set -euo pipefail

: "${DATABASE_OWNER_URL:?required}"

# env var instead of --database-url keeps credentials out of the process list
DATABASE_URL="$DATABASE_OWNER_URL" sqlx migrate run --source "$(dirname "$0")/../backend/migrations"
