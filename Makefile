.PHONY: help setup migrate seed test health logs dev test-backend

help:
	@echo "DUITGOO Developer Commands"
	@echo ""
	@echo "make setup    - Install dependencies and prepare local environment"
	@echo "make migrate  - Run database migrations"
	@echo "make seed     - Seed database with initial data"
	@echo "make test     - Run all tests (frontend, rust, python, etc.)"
	@echo "make test-backend - Run backend tests"
	@echo "make health   - Run health checks across all services"
	@echo "make logs     - View aggregated logs"
	@echo "make dev      - Start local development environment"

setup:
	@echo "Setting up local environment..."
	cargo install sqlx-cli --no-default-features --features rustls,postgres

migrate:
	@echo "Running database migrations..."
	cd backend/services/identity && sqlx database create -D postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db || true
	cd backend/services/identity && sqlx migrate run -D postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db
	cd backend/services/business && sqlx migrate run -D postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db
	cd backend/services/accounting && sqlx migrate run -D postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db
	cd backend/services/transaction && sqlx migrate run -D postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db

seed:
	@echo "Seeding database..."
	@echo "Not implemented yet"

test: test-backend
	@echo "Running frontend tests..."
	cd frontend && npm run test:unit
	@echo "Running intelligence tests..."
	cd intelligence && pytest || true

test-backend:
	@echo "Running backend tests..."
	cd backend && cargo test

health:
	@echo "Running health checks..."
	curl -s http://localhost:8000/healthz

logs:
	@echo "Viewing logs..."
	docker compose logs -f

dev:
	@echo "Starting development environment..."
	docker compose up -d
