use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, ClientBuilder};

use crate::GazelleClient;

/// The number of requests allowed per duration
const ALLOWED_REQUESTS_PER_DURATION: u64 = 10;
const REQUEST_LIMIT_DURATION: Duration = Duration::from_secs(10);

/// Create a [`GazelleClient`]
pub struct GazelleClientFactory {
    key: String,
    url: String,
    user_agent: String,
}

impl GazelleClientFactory {
    #[must_use]
    pub fn create(&self) -> GazelleClient {
        let client = self.create_client();
        let rate_limited_client = tower::ServiceBuilder::new()
            .rate_limit(ALLOWED_REQUESTS_PER_DURATION, REQUEST_LIMIT_DURATION)
            .service(client);
        GazelleClient {
            api_url: self.url.clone(),
            client: rate_limited_client,
        }
    }

    fn create_client(&self) -> Client {
        ClientBuilder::new()
            .default_headers(self.get_headers())
            .build()
            .expect("Client builder should not fail")
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            HeaderValue::try_from(&self.user_agent).expect("user agent should not fail"),
        );
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(header::AUTHORIZATION, self.get_authorization());
        headers
    }

    fn get_authorization(&self) -> HeaderValue {
        let mut value =
            HeaderValue::try_from(self.key.clone()).expect("Authorization header should not fail");
        value.set_sensitive(true);
        value
    }
}
