# Roadmap

*Note: As the project is in early development, this roadmap represents the proposed trajectory based on the current architectural skeleton.*

## Completed
- Monorepo structure setup.
- Basic Rust/Axum microservice boilerplate generation.
- Database skeleton for core domains (Accounting, Identity, Sales, etc.).
- Global Makefile for local development orchestration.
- Frontend Vue 3 + Tailwind skeleton.
- Local Observability stack (Docker Compose).

## In Progress
- Implementation of the Identity Service (hashing implemented, JWT pending).
- Implementation of the Accounting ledger service (schemas exist).
- Frontend routing and authentication guards.

## Planned
- API Gateway implementation (routing and OpenTelemetry tracing).
- NATS JetStream event publishing and subscribing for inter-service communication.
- Python Intelligence service integration (OCR and forecasting).
- Comprehensive end-to-end testing suite.
- CI/CD pipeline automation (GitHub Actions).
