use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GazelleClientOptions {
    pub user_agent: String,
    pub key: String,
    pub url: String,
}
