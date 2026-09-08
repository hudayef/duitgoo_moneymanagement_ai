# DUITGOO

Production-Grade Financial Intelligence Platform for UMKM.

> **RECORD → UNDERSTAND → PREDICT → DECIDE**

Duitgoo is a polyglot microservices financial platform offering double-entry accounting capabilities coupled with AI-driven intelligence (Python), a high-performance backend API (Rust), and a modern web frontend (Vue 3).

## Features
- **Double-Entry Accounting**: Immutable ledger ensuring strict financial compliance.
- **Microservices Architecture**: Isolated Rust/Axum services communicating asynchronously via NATS JetStream.
- **Financial Intelligence**: Python-based AI engine for insights, ML forecasting, and OCR (not currently implemented fully).
- **Observability**: Full suite using OpenTelemetry, Loki, Tempo, and Grafana.

## Architecture & Tech Stack
- **Frontend**: Vue 3, TypeScript, Vite, Pinia, TailwindCSS, shadcn-vue.
- **Backend**: Rust, Axum, Tokio, SQLx.
- **Intelligence**: Python, FastAPI, Pydantic, Scikit-learn, Pandas.
- **Databases**: PostgreSQL (primary store per service), Redis (caching).
- **Messaging**: NATS JetStream (events).

## Documentation Index

| Document | Description |
|---|---|
| [Getting Started](docs/getting-started.md) | Setup project & local execution |
| [Architecture](docs/architecture.md) | System architecture & workflows |
| [Project Structure](docs/project-structure.md) | Folder organization and logic |
| [Database](docs/database.md) | Schema design and engine constraints |
| [API](docs/api/README.md) | API reference |
| [Development](docs/development.md) | Development workflows and commands |
| [Testing](docs/testing.md) | Automated testing procedures |
| [Deployment](docs/deployment.md) | Deployment processes and configuration |
| [Security](docs/security.md) | Security implementation and guidelines |
| [Troubleshooting](docs/troubleshooting.md) | Common errors and resolutions |
| [Contributing](docs/contributing.md) | Contribution guide |
| [Roadmap](docs/roadmap.md) | Future development plans |
| [FAQ](docs/faq.md) | Frequently asked questions |

## Quick Start
```bash
# Start local infrastructure
docker compose up -d

# Setup environment and run migrations
make setup
make migrate

# Start the frontend dev server
cd frontend && pnpm dev
```
