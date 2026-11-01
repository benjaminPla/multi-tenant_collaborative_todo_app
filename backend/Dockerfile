FROM rust:1.98-slim-bookworm AS base
WORKDIR /app

# ── migrate ──────────────────────────────────────────────────────────────
FROM base AS sqlx
RUN cargo install sqlx-cli --locked --no-default-features --features postgres --version ^0.9

FROM gcr.io/distroless/cc-debian12:nonroot AS migrate
COPY --from=sqlx /usr/local/cargo/bin/sqlx /sqlx
COPY migrations /migrations
ENTRYPOINT ["/sqlx", "migrate", "run", "--source", "/migrations"]

# ── build ────────────────────────────────────────────────────────────────
FROM base AS build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --locked && rm -rf src
COPY src src
RUN touch src/main.rs && cargo build --release --locked

# ── runtime ──────────────────────────────────────────────────────────────
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=build /app/target/release/backend /backend
EXPOSE 3000
ENTRYPOINT ["/backend"]
