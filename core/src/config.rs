use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RakuenConfig {
    pub host: String,
    pub port: u16,
    pub secret: String,
}

impl Default for RakuenConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 7590,
            secret: "secret".to_string(),
        }
    }
}
