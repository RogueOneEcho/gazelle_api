use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, ClientBuilder};

use crate::{GazelleClient, GazelleClientOptions};

/// The number of requests allowed per duration
const ALLOWED_REQUESTS_PER_DURATION: u64 = 10;
const REQUEST_LIMIT_DURATION: Duration = Duration::from_secs(10);

/// Create a [`GazelleClient`]
pub struct GazelleClientFactory {
    pub options: GazelleClientOptions,
}

impl GazelleClientFactory {
    #[must_use]
    pub fn create(self) -> GazelleClient {
        let GazelleClientOptions { user_agent, key, url} = self.options;
        let client = create_client(user_agent, key);
        let rate_limited_client = tower::ServiceBuilder::new()
            .rate_limit(ALLOWED_REQUESTS_PER_DURATION, REQUEST_LIMIT_DURATION)
            .service(client);
        GazelleClient {
            base_url: url,
            client: rate_limited_client,
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
    let mut value =
        HeaderValue::try_from(key).expect("Authorization header should not fail");
    value.set_sensitive(true);
    value
}
