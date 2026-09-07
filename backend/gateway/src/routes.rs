use axum::{
    routing::{get, any},
    Router,
    extract::{Request, State},
    response::Response,
};
use reqwest::Client;
use shared::errors::AppError;

#[derive(Clone)]
pub struct GatewayState {
    pub client: Client,
    pub identity_url: String,
}

pub fn create_router(state: GatewayState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/auth/*path", any(proxy_identity))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "OK"
}

// Basic reverse proxy to identity service
async fn proxy_identity(
    State(state): State<GatewayState>,
    req: Request,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

    // Rewrite path: /api/v1/auth/login -> /login on identity service
    let downstream_path = path.strip_prefix("/api/v1/auth").unwrap_or(path);
    let target_url = format!("{}{}{}", state.identity_url, downstream_path, query);

    let (parts, body) = req.into_parts();

    let mut client_req = state.client.request(parts.method, target_url);

    // Forward headers
    for (name, value) in parts.headers.iter() {
        // Strip host header so reqwest sets it correctly
        if name != axum::http::header::HOST {
            client_req = client_req.header(name.clone(), value.clone());
        }
    }

    // Convert axum body to reqwest body
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
         Ok(b) => b,
         Err(_) => return Err(AppError::InternalError("Failed to read request body".to_string())),
    };

    let client_req = client_req.body(body_bytes);

    let res = match client_req.send().await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            return Err(AppError::InternalError("Downstream service unavailable".to_string()));
        }
    };

    let mut response_builder = Response::builder().status(res.status());

    // Copy headers back
    if let Some(headers) = response_builder.headers_mut() {
        for (name, value) in res.headers().iter() {
             headers.insert(name.clone(), value.clone());
        }
    }

    let bytes = match res.bytes().await {
        Ok(b) => b,
        Err(_) => return Err(AppError::InternalError("Failed to read response body".to_string())),
    };

    Ok(response_builder.body(axum::body::Body::from(bytes)).unwrap())
}
