use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration options for creating a [`GazelleClient`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GazelleClientOptions {
    /// User-Agent header value for HTTP requests
    pub user_agent: String,
    /// API key for authentication
    pub key: String,
    /// Base URL of the Gazelle indexer.
    ///
    /// Example: `https://orpheus.network`
    pub url: String,
    /// Maximum requests allowed per duration.
    ///
    /// Default: `5`
    pub requests_allowed_per_duration: Option<usize>,
    /// Duration before the rate limit resets.
    ///
    /// Default: `10` seconds
    pub request_limit_duration: Option<Duration>,
}
