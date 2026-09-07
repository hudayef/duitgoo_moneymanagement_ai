use async_nats::jetstream::{self, context::Context};
use serde::Serialize;
use crate::errors::AppError;

pub struct EventPublisher {
    js: Context,
}

impl EventPublisher {
    pub async fn new(nats_url: &str) -> Result<Self, AppError> {
        let client = async_nats::connect(nats_url).await.map_err(|e| {
            tracing::error!("Failed to connect to NATS: {}", e);
            AppError::InternalError("Messaging unavailable".to_string())
        })?;

        let js = jetstream::new(client);

        Ok(Self { js })
    }

    pub async fn publish<T: Serialize>(&self, subject: &str, event: &T) -> Result<(), AppError> {
        let payload = serde_json::to_vec(event).map_err(|e| {
            tracing::error!("Failed to serialize event: {}", e);
            AppError::InternalError("Event serialization failed".to_string())
        })?;

        self.js.publish(subject.to_string(), payload.into()).await.map_err(|e| {
            tracing::error!("Failed to publish event: {}", e);
            AppError::InternalError("Event publishing failed".to_string())
        })?;

        Ok(())
    }
}
