use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
    pub nats_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<AppConfig>()
    }
}
