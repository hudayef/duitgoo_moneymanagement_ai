# Authentication

Authentication is handled by the **Identity Service**.

## Current Implementation
- Passwords are hashed securely utilizing the `argon2` crate.
- The registration flow accepts an email and password and returns the created user entity.

*Note: Login endpoints, JWT token generation, refresh tokens, and robust role-based access control (RBAC) are not currently implemented / not found in the repository.*

## Frontend Integration
The frontend utilizes a Pinia store (`useAuthStore`) to manage user state. Vue Router employs navigation guards to protect routes:
- `meta: { requiresAuth: true }` redirects unauthenticated users to `/login`.
- `meta: { guestOnly: true }` redirects authenticated users away from the login page to the dashboard.
