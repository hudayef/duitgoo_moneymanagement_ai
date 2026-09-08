# Deployment

*Note: Automated deployment (CI/CD) pipelines and production infrastructure configurations (e.g., Kubernetes manifests, Terraform) are not currently implemented / not found in the repository.*

## Environment Configuration
Services currently fallback to hardcoded default strings (e.g., `postgres://duitgoo_user:duitgoo_password@localhost:5432/...`) if environment variables are missing.

For a production deployment, the following environment variables **must** be securely injected into the containers:

| Variable | Required | Description | Secret |
|---|---|---|---|
| `DATABASE_URL` | Yes | Connection string for the service's specific PostgreSQL database. | Yes |
| `SERVER_PORT` | No | Overrides the default port for the Rust Axum service. | No |
| `NATS_URL` | Yes | Connection string for the NATS JetStream cluster. | No |

## Containerization
Currently, only infrastructure dependencies (PostgreSQL, Redis, NATS, Observability stack) are containerized via `docker-compose.yml` for local development.

Dockerfiles for the Frontend (Nginx/Vite), Backend Microservices (Rust), and Intelligence Engine (Python) are pending creation.
