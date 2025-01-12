use colored::Colorize;
use log::*;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::SystemTime;

use crate::*;

/// A client for the Gazelle API
///
/// Created by an [`GazelleClientFactory`]
pub struct GazelleClient {
    pub base_url: String,
    pub client: Client,
    pub limiter: RateLimiter,
}

impl From<GazelleClientOptions> for GazelleClient {
    /// Create a [`GazelleClient`] from [`GazelleClientOptions`]
    #[must_use]
    fn from(options: GazelleClientOptions) -> GazelleClient {
        let factory = GazelleClientFactory { options };
        factory.create()
    }
}

impl GazelleClient {
    pub(crate) async fn get<T: DeserializeOwned>(
        &mut self,
        query: String,
    ) -> Result<T, GazelleError> {
        let result = self.get_internal(query).await;
        handle_result(result).await
    }

    pub(crate) async fn get_internal(&mut self, query: String) -> Result<Response, reqwest::Error> {
        self.limiter.execute();
        let path = format!("/ajax.php?{query}");
        trace!("{} request GET {path}", "Sending".bold());
        let url = format!("{}{path}", self.base_url);
        let start = SystemTime::now();
        let result = self.client.get(url).send().await;
        let elapsed = start
            .elapsed()
            .expect("elapsed should not fail")
            .as_secs_f64();
        trace!("{} response after {elapsed:.3}", "Received".bold());
        result
    }
}

pub(crate) async fn handle_result<T: DeserializeOwned>(
    result: Result<Response, reqwest::Error>,
) -> Result<T, GazelleError> {
    let (status_code, json) = get_response(result).await?;
    let response = deserialize(json)?;
    get_result(status_code, response)
}

pub(crate) async fn get_response(
    result: Result<Response, reqwest::Error>,
) -> Result<(StatusCode, String), GazelleError> {
    let response = result.map_err(GazelleError::request)?;
    let status_code = response.status();
    let json = response.text().await.map_err(GazelleError::response)?;
    Ok((status_code, json))
}

pub(crate) fn deserialize<T: DeserializeOwned>(
    json: String,
) -> Result<ApiResponse<T>, GazelleError> {
    // Remove malformed OPS response
    let json = json.replace("\"response\":[],", "");
    serde_json::from_str(&json).map_err(GazelleError::deserialization)
}

pub(crate) fn get_result<T: DeserializeOwned>(
    status_code: StatusCode,
    response: ApiResponse<T>,
) -> Result<T, GazelleError> {
    if let Some(error) = GazelleError::match_client_error(status_code) {
        trace!("Response status code indicates {error:?}: {status_code}");
        return Err(error);
    }
    if let Some(message) = response.error {
        if let Some(error) = GazelleError::match_response_error(message.as_str()) {
            trace!("Response error message indicates {error:?}: {message}");
            return Err(error);
        }
        warn!("Response error message was unexpected: {message}");
        return Err(GazelleError::unexpected(status_code, message));
    }
    response.response.ok_or_else(|| {
        warn!("Response did not contain an error or response object: {status_code}");
        GazelleError::empty(status_code)
    })
}
