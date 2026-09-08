# Architecture

DUITGOO is built using a polyglot microservices architecture optimized for financial intelligence and strict ledger management.

## High-Level Architecture
```mermaid
flowchart TD
    Client[Web Client (Vue 3)] --> APIGateway[API Gateway (Rust/Axum)]
    APIGateway --> Services[Microservices (Rust)]
    APIGateway --> Intelligence[Intelligence Engine (Python)]

    subgraph Microservices
        Identity[Identity Service]
        Accounting[Accounting Service]
        Business[Business Service]
        Transaction[Transaction Service]
        Reporting[Reporting Service]
        Sales[Sales Service]
        Finance[Finance Service]
    end

    Services --> DB[(PostgreSQL Databases)]
    Services --> NATS[NATS JetStream]
    NATS --> Services

    Intelligence --> NATS

    Services --> Observability[OpenTelemetry / Loki / Tempo]
```

## Core Components
- **Frontend**: A Vue 3 SPA built with Vite, TypeScript, and Pinia. It communicates strictly through the public API/API Gateway.
- **Backend Microservices**: Written in Rust (Axum, Tokio). Each domain (e.g., identity, accounting, sales) is isolated into its own microservice.
- **Event-Driven Messaging**: Asynchronous inter-service communication is achieved via NATS JetStream using the `EventPublisher` and `EventSubscriber` patterns from the `backend/shared` crate.
- **Intelligence Engine**: A Python (FastAPI) service responsible for ML and data science workloads. **Constraint:** The Python AI service must never directly post accounting truth or invent financial numbers. It only provides insights or read-only inferences.
- **Data Stores**:
  - Each microservice owns its PostgreSQL database/schema. **Constraint:** Services must never share database tables.
  - Redis is used for caching operations (not fully implemented in all services).
- **Observability**: A robust pipeline relying on OpenTelemetry Collector, feeding logs to Loki, traces to Tempo, and visualized in Grafana.

## Key Architectural Rules
- **Double-Entry Accounting is mandatory**. All money uses `DECIMAL(18,2)`. Debits must always equal credits.
- **Posted entries are immutable**. To correct a mistake, a reversing/voiding entry must be created.
- The **Reporting Service** utilizes a CQRS-like pattern, storing read-optimized financial reports using `JSONB` in PostgreSQL to isolate heavy read query loads from primary transactional databases.
