# API Endpoints

*Note: The comprehensive list of REST endpoints is currently under development. Please refer to the OpenAPI specification (`contracts/openapi/duitgoo.yaml`) for the most up-to-date definitions.*

## Identity Service

### POST /register

Registers a new user in the system.

**Authentication:** None

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:**
Returns the newly created user entity.

**Errors:**
- `400 BAD_REQUEST`: Email and password are required.
