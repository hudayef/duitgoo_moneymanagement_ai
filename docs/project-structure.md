# Project Structure

The repository is structured as a monorepo containing all components of the DUITGOO platform.

```text
.
├── backend/
│   ├── Cargo.toml            # Workspace definition for all Rust projects
│   ├── gateway/              # The API Gateway (Rust/Axum) handling reverse proxy and tracing
│   ├── services/             # Microservices
│   │   ├── accounting/       # Manages the immutable ledger and journals
│   │   ├── audit/
│   │   ├── budget/
│   │   ├── business/
│   │   ├── finance/
│   │   ├── identity/         # User authentication and registration
│   │   ├── inventory/
│   │   ├── notification/
│   │   ├── purchase/
│   │   ├── reporting/        # CQRS read models for financial reports
│   │   ├── sales/
│   │   ├── subscription/
│   │   └── transaction/
│   ├── shared/               # Shared Rust crate for common utilities (errors, config, logging, OpenTelemetry, NATS)
│   └── tests/
├── contracts/
│   ├── events/               # JSON schemas defining NATS event payloads (e.g., base_event.json)
│   └── openapi/              # OpenAPI/Swagger definitions for the API
├── frontend/
│   ├── package.json          # Frontend dependencies (manage with pnpm ONLY)
│   ├── src/
│   │   ├── assets/
│   │   ├── layouts/          # Global UI layouts (e.g., DashboardLayout.vue)
│   │   ├── pages/            # View components mapping to routes
│   │   ├── router/           # Vue Router configuration (includes navigation guards)
│   │   ├── stores/           # Pinia state management (e.g., useAuthStore)
│   │   └── utils/
│   └── tests/                # Vitest unit testing suite
├── infrastructure/
│   └── observability/        # Configurations for Grafana, Loki, Tempo, and OpenTelemetry
├── intelligence/
│   ├── main.py               # FastAPI entrypoint for the Python AI engine
│   └── requirements.txt      # Python dependencies
├── docker-compose.yml        # Local infrastructure definition
├── Makefile                  # Global developer commands
└── README.md                 # Primary entrypoint
```
