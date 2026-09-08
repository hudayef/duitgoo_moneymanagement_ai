# Development Guide

## Workflow Requirements
- **Phase-by-Phase**: Work strictly phase by phase and implement the smallest complete slice.
- **Frontend Package Manager**: You MUST use `pnpm` exclusively for all frontend dependency management. Never use `npm` or `yarn`.

## Local Development Commands
The global `Makefile` orchestrates development commands across the monorepo:

| Command | Description |
|---|---|
| `make setup` | Installs global tools (e.g., `sqlx-cli`). |
| `make migrate` | Creates local databases and runs migrations for all Rust microservices. |
| `make test` | Runs tests across the frontend, backend, and intelligence services. |
| `make health` | Pings the API Gateway healthcheck endpoint. |
| `make logs` | Tails Docker Compose logs for infrastructure services. |
| `make dev` | Starts local infrastructure via Docker Compose. |

## Frontend Development
The frontend is built with Vue 3, Vite, and TailwindCSS.
- Run dev server: `pnpm dev`
- Run linting (ESLint): `pnpm lint` *(Note: requires configuration setup if missing)*
- Run formatting (Prettier): `pnpm format`
- Run unit tests (Vitest): `pnpm test:unit --run`
- Build for production: `pnpm build`

### Frontend UI/UX Standards
- Accessibility is mandatory (e.g., proper `aria-labels`, `focus-visible` styling for keyboard navigation, and "Skip to main content" links).
- Authentication logic is handled by `useAuthStore` in Pinia, working in tandem with Vue Router navigation guards (`meta: { requiresAuth: true }`).

## Backend (Rust) Development
- Shared logic (errors, environment configuration, NATS messaging, tracing) is housed in the `backend/shared` workspace crate.
- Every service must expose the following endpoints: `/healthz`, `/readyz`, `/metrics`, and `/version`.

## Intelligence (Python) Development
- Requires Python 3.
- Fast API application located in the `intelligence/` directory.
- `pip install -r requirements.txt`
- `pytest` for testing.
