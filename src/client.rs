use colored::Colorize;
use log::*;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::SystemTime;

use crate::GazelleError::Deserialization;
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

async fn get_response(
    result: Result<Response, reqwest::Error>,
) -> Result<(StatusCode, String), GazelleError> {
    let response = result.map_err(GazelleError::Request)?;
    let status_code = response.status();
    let json = response.text().await.map_err(GazelleError::Request)?;
    Ok((status_code, json))
}

fn deserialize<T: DeserializeOwned>(json: String) -> Result<ApiResponse<T>, GazelleError> {
    // Remove malformed OPS response
    let json = json.replace("\"response\":[],", "");
    serde_json::from_str(&json).map_err(|e| Deserialization(e, json))
}

fn get_result<T: DeserializeOwned>(
    status_code: StatusCode,
    response: ApiResponse<T>,
) -> Result<T, GazelleError> {
    if let Some(error) = GazelleError::from_status_code(status_code) {
        return Err(error);
    }
    if let Some(error) = response.error {
        if let Some(error) = GazelleError::from_str(error.as_str()) {
            return Err(error);
        }
        if !status_code.is_success() {
            return Err(GazelleError::Other(status_code, error));
        }
    }
    response
        .response
        .ok_or(GazelleError::Other(status_code, "No response.".to_owned()))
}
