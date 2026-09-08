# Getting Started

Follow these steps to run DUITGOO on your local machine.

## Prerequisites
Ensure the following tools are installed on your machine:
- **Docker** and **Docker Compose**
- **Rust** (cargo)
- **Node.js** and **pnpm** (used exclusively for frontend dependencies)
- **Python 3** (for the intelligence service)
- **Make**

## 1. Clone Repository
```bash
git clone https://github.com/yourusername/duitgoo.git
cd duitgoo
```

## 2. Infrastructure Setup
Start the local infrastructure (PostgreSQL, Redis, NATS, and Observability stack) via Docker Compose:
```bash
docker compose up -d
```

## 3. Install Dependencies
Run the global setup script to install tools like `sqlx-cli`:
```bash
make setup
```

For the frontend:
```bash
cd frontend
pnpm install
```

For the intelligence service:
```bash
cd intelligence
pip install -r requirements.txt
```

## 4. Setup Databases and Run Migrations
DUITGOO's microservices utilize isolated databases. Run the setup target to create and migrate all databases:
```bash
make migrate
```
*Note: Make sure your `postgres` Docker container has finished initializing before running migrations.*

## 5. Verify Installation and Health
Verify that the services are healthy. Note that `make health` relies on services running locally.
```bash
make health
```
To run tests across all layers (frontend, backend, intelligence):
```bash
make test
```

## 6. Start Development Servers
Frontend:
```bash
cd frontend
pnpm dev
```
Backend/Intelligence:
Refer to [Development Guide](development.md) for individual service execution.
