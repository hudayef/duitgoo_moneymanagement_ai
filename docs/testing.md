# Testing

DUITGOO enforces testing across all layers of the stack.

You can run the entire suite using the global Makefile:
```bash
make test
```

## Frontend Testing
The frontend utilizes **Vitest** for unit testing.
```bash
cd frontend
pnpm test:unit
```

## Backend Testing
The Rust backend utilizes the standard `cargo test` framework.
```bash
cd backend
cargo test
```
*Alternatively, you can run `make test-backend` from the project root.*

## Intelligence Testing
The Python intelligence engine utilizes **Pytest**.
```bash
cd intelligence
pytest
```

*Note: E2E (End-to-End) and dedicated API integration testing frameworks are not currently implemented / not found in the repository.*
