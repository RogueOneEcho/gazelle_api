use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GazelleClientOptions {
    pub key: String,
    pub url: String,
}
