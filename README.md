# DUITGOO - AI-powered financial intelligence platform

Production-Grade Financial Intelligence Platform for UMKM.

**RECORD → UNDERSTAND → PREDICT → DECIDE**

## Architecture

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust + Axum + Tokio + SQLx (Microservices)
- **Intelligence**: Python (AI, ML, forecasting, OCR)
- **Data**: PostgreSQL, Redis
- **Messaging**: NATS JetStream
- **Observability**: OpenTelemetry + Loki + Tempo + Grafana
- **Deployment**: Docker + CI/CD

## Rules

- Double-entry accounting is mandatory.
- Frontend does not determine financial truth.
- Python AI does not write directly to the ledger.
- API-first with REST API versioned.
- Services never share database tables.

## Local Development

```bash
docker compose up -d
make setup
make test
make health
make dev
```
