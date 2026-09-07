pub mod publisher;
pub mod subscriber;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseEvent<T> {
    pub event_id: Uuid,
    pub event_type: String,
    pub version: i32,
    pub timestamp: DateTime<Utc>,
    pub trace_id: Option<String>,
    pub business_id: Uuid,
    pub payload: T,
}

impl<T> BaseEvent<T> {
    pub fn new(event_type: &str, business_id: Uuid, payload: T, trace_id: Option<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            version: 1,
            timestamp: Utc::now(),
            trace_id,
            business_id,
            payload,
        }
    }
}
