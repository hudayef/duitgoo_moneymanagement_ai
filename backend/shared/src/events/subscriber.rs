use async_nats::jetstream::{self, context::Context, stream::Stream};
use crate::errors::AppError;

pub struct EventSubscriber {
    js: Context,
}

impl EventSubscriber {
    pub async fn new(nats_url: &str) -> Result<Self, AppError> {
        let client = async_nats::connect(nats_url).await.map_err(|e| {
            tracing::error!("Failed to connect to NATS: {}", e);
            AppError::InternalError("Messaging unavailable".to_string())
        })?;

        let js = jetstream::new(client);

        Ok(Self { js })
    }

    pub async fn get_stream(&self, stream_name: &str) -> Result<Stream, AppError> {
        self.js.get_stream(stream_name).await.map_err(|e| {
             tracing::error!("Failed to get stream: {}", e);
             AppError::InternalError("Messaging stream unavailable".to_string())
        })
    }
}
