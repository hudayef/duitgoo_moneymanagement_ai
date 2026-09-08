# Security

## Implemented
- **Password Hashing**: The Identity Service utilizes strong, industry-standard `argon2` hashing for user passwords.
- **Microservice Isolation**: Databases are strictly segregated by service domain, limiting blast radius in the event of a compromise.
- **Frontend State Security**: Vue Router navigation guards prevent unauthorized access to authenticated client-side routes.

## Recommended Improvements
*The following security features are not currently implemented / not found in the repository and should be prioritized:*
- **Authentication Strategy**: Implement JWT issuing and validation in the Identity Service.
- **API Gateway Protection**: Add rate limiting, CORS configuration, and JWT validation middleware to the API Gateway.
- **Secret Management**: Migrate away from plain-text fallback strings in code to a robust secrets manager (e.g., HashiCorp Vault, AWS Secrets Manager) and enforce strict `.env` file usage in local development.
- **Input Validation**: Ensure rigorous validation logic is applied to all incoming API requests (e.g., using `validator` crate in Rust).
- **Network Security**: Enforce TLS/HTTPS for all external communication and consider mTLS for inter-service communication within the cluster.
