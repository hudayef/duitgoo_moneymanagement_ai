# API Documentation

The DUITGOO API uses REST principles and communicates via JSON.

All external traffic should route through the API Gateway, which acts as a reverse proxy and is responsible for generating trace and request IDs for observability.

## Standard Endpoints
By architectural rule, **every microservice** exposes the following base endpoints:

| Endpoint | Method | Description |
|---|---|---|
| `/healthz` | GET | Liveness probe. Returns a generic OK status. |
| `/readyz` | GET | Readiness probe. Validates database/dependency connectivity. |
| `/metrics` | GET | Prometheus-compatible metrics endpoint. |
| `/version` | GET | Returns the current service version. |

## Documentation Sections
- [Authentication](authentication.md)
- [Endpoints](endpoints.md) (Currently under development)
- [Errors](errors.md)

*A partial OpenAPI definition is available at `contracts/openapi/duitgoo.yaml`.*
