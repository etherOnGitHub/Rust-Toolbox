use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub component: String,
    pub healthy: bool,
}

impl Health {
    pub fn new(component: impl Into<String>, healthy: bool) -> Self {
        Self {
            component: component.into(),
            healthy,
        }
    }

    pub fn summary(&self) -> &'static str {
        if self.healthy {
            "ok"
        } else {
            "degraded"
        }
    }
}
