use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: "dev".to_string(),
        }
    }
}
