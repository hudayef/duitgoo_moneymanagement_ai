# FAQ

**Q: Why does DUITGOO use so many microservices?**
A: DUITGOO enforces strict domain boundaries to manage complex financial logic. This polyglot architecture allows the transactional core (Rust) to scale independently from the heavy data science workloads (Python).

**Q: Can the Python AI engine post directly to the accounting ledger?**
A: No. By strict architectural rule, the AI service provides intelligence and insights, but it must never directly invent financial numbers or post to the immutable ledger.

**Q: Why does my frontend build fail with ESLint errors?**
A: The repository currently lacks an `eslint.config.js` file for ESLint v9+. You may need to create this configuration locally or bypass linting temporarily during development.

**Q: Where is the CI/CD configuration?**
A: Automated deployment pipelines (like GitHub Actions workflows) are not currently implemented in the repository.

**Q: How does authentication work?**
A: Currently, the Identity service handles user creation and password hashing (argon2). Full JWT issuing, validation middleware at the API Gateway, and Role-Based Access Control are pending implementation.
