# API Errors

The backend services utilize a unified error handling approach via the `shared::errors::AppError` abstraction.

When an error occurs, the API returns a standardized JSON structure:

```json
{
  "error_code": "STRING_CODE",
  "message": "Human readable message"
}
```

## Standard Error Codes

| HTTP Status | Error Code | Description |
|---|---|---|
| 400 | `BAD_REQUEST` | Client provided invalid input or failed validation. |
| 401 | `UNAUTHORIZED` | Client lacks valid authentication credentials. |
| 404 | `NOT_FOUND` | The requested resource does not exist. |
| 500 | `INTERNAL_ERROR` | An unexpected server-side error occurred. |
