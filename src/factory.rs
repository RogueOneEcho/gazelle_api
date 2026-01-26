use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, header};

use crate::{GazelleClient, GazelleClientOptions, RateLimiter};

const DEFAULT_LIMIT: usize = 5;
const DEFAULT_LIMIT_DURATION: Duration = Duration::from_secs(10);

/// Factory for creating a [`GazelleClient`]
pub struct GazelleClientFactory {
    /// Configuration options for the client
    pub options: GazelleClientOptions,
}

impl GazelleClientFactory {
    /// Create a new [`GazelleClient`] from the configured options
    #[must_use]
    pub fn create(self) -> GazelleClient {
        let GazelleClientOptions {
            user_agent,
            key,
            url: base_url,
            requests_allowed_per_duration: num,
            request_limit_duration: per,
        } = self.options;
        let client = create_client(user_agent, key);
        let limiter = RateLimiter::new(
            num.unwrap_or(DEFAULT_LIMIT),
            per.unwrap_or(DEFAULT_LIMIT_DURATION),
        );
        GazelleClient {
            base_url,
            client,
            limiter,
        }
    }
}

fn create_client(user_agent: String, key: String) -> Client {
    ClientBuilder::new()
        .default_headers(get_headers(user_agent, key))
        .build()
        .expect("Client builder should not fail")
}

fn get_headers(user_agent: String, key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::try_from(user_agent).expect("user agent should not fail"),
    );
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(header::AUTHORIZATION, get_authorization(key));
    headers
}

fn get_authorization(key: String) -> HeaderValue {
    let mut value = HeaderValue::try_from(key).expect("Authorization header should not fail");
    value.set_sensitive(true);
    value
}
