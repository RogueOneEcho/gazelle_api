use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GazelleClientOptions {
    pub user_agent: String,
    pub key: String,
    pub url: String,
    pub requests_allowed_per_duration: Option<usize>,
    pub request_limit_duration: Option<Duration>,
}
