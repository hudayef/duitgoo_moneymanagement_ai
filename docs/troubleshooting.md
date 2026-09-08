# Troubleshooting

## Problem: Port 3000 is in use
### Symptoms
When starting the frontend using `pnpm dev`, Vite reports `Port 3000 is in use, trying another one...` and starts on `3001` or fails.
### Cause
Grafana (configured in `docker-compose.yml`) also defaults to port 3000.
### Solution
Either access the frontend on the alternative port Vite provided, or update `frontend/vite.config.ts` (or Grafana's docker config) to use a distinct port to avoid conflicts.

## Problem: Database migration failures
### Symptoms
Running `make migrate` throws errors about tracking collisions or failed connections.
### Cause
1. The Postgres container may not have fully initialized before the migration script ran.
2. Multiple microservices might be trying to run migrations with the exact same timestamp prefix on the shared local database server, causing collisions in the `_sqlx_migrations` table.
### Solution
1. Ensure `docker-compose ps` shows Postgres as healthy.
2. Ensure SQLx migration file timestamps are uniquely sequenced (e.g., staggered by day) across all microservices.

## Problem: NATS connection refused
### Symptoms
Backend services fail to start, citing "Messaging unavailable" or "Failed to connect to NATS".
### Cause
The NATS container is not running, or the `NATS_URL` environment variable is misconfigured.
### Solution
Run `docker compose up -d` to ensure the NATS server is running. Check `docker compose logs nats` for startup errors.

## Problem: ESLint errors preventing frontend build
### Symptoms
Running `pnpm lint` or `pnpm build` fails indicating a missing `eslint.config.js`.
### Cause
The frontend repository lacks the proper ESLint flat configuration file required by ESLint v9+.
### Solution
Create an `eslint.config.js` file in the `frontend` directory configuring your Vue 3 and TypeScript linting rules.
