# Runtime

Date: 2026-10-31

## Options

**Long-running server (chosen)**
- One long-running Axum process.
- Persistent SSE/WebSocket connections and one Postgres pool per process.

**Serverless (rejected)**
- No persistent process for SSE/WebSockets; needs an external connection manager (e.g. API Gateway WebSocket).
- Connection storms against Postgres; needs a pooler (e.g. RDS Proxy, PgBouncer).
- Cold starts add latency.

## Consequences

- Simple deploy: one binary, one process.
- Real-time fan-out held in process memory only reaches clients on the same instance; scaling out needs a shared bus (e.g. Postgres `LISTEN/NOTIFY`, Redis).
