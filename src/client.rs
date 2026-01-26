use async_trait::async_trait;
use colored::Colorize;
use log::*;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::SystemTime;

use crate::*;

/// A client for the Gazelle API
///
/// Created by a [`GazelleClientFactory`] or via [`From<GazelleClientOptions>`].
pub struct GazelleClient {
    /// Base URL of the Gazelle indexer.
    ///
    /// Example: `https://orpheus.network`
    pub base_url: String,
    /// HTTP client with configured headers for authentication
    pub client: Client,
    /// Rate limiter to throttle API requests
    pub limiter: RateLimiter,
}

impl From<GazelleClientOptions> for GazelleClient {
    /// Create a [`GazelleClient`] from [`GazelleClientOptions`]
    fn from(options: GazelleClientOptions) -> GazelleClient {
        let factory = GazelleClientFactory { options };
        factory.create()
    }
}

impl GazelleClient {
    pub(crate) async fn get<T: DeserializeOwned>(&self, query: String) -> Result<T, GazelleError> {
        let result = self.get_internal(query).await;
        handle_result(result).await
    }

    pub(crate) async fn get_internal(&self, query: String) -> Result<Response, reqwest::Error> {
        self.limiter.execute().await;
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
    if let Some(message) = &response.error {
        trace!(
            "{} {status_code} response with error: {message}",
            "Received".bold()
        );
        if let Some(error) = GazelleError::match_response_error(message) {
            return Err(error);
        }
    } else {
        trace!("{} {status_code} response without error", "Received".bold());
    }
    if let Some(error) = GazelleError::match_status_error(status_code, response.error.clone()) {
        return Err(error);
    }
    response
        .response
        .ok_or_else(|| GazelleError::other(status_code, response.error))
}

#[async_trait]
impl GazelleClientTrait for GazelleClient {
    async fn get_torrent(&self, id: u32) -> Result<TorrentResponse, GazelleError> {
        GazelleClient::get_torrent(self, id).await
    }

    async fn get_torrent_group(&self, id: u32) -> Result<GroupResponse, GazelleError> {
        GazelleClient::get_torrent_group(self, id).await
    }

    async fn get_user(&self, id: u32) -> Result<User, GazelleError> {
        GazelleClient::get_user(self, id).await
    }

    async fn download_torrent(&self, id: u32) -> Result<Vec<u8>, GazelleError> {
        GazelleClient::download_torrent(self, id).await
    }

    async fn upload_torrent(&self, upload: UploadForm) -> Result<UploadResponse, GazelleError> {
        GazelleClient::upload_torrent(self, upload).await
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::GazelleError::*;

    // deserialize tests

    #[test]
    fn deserialize_success_response() {
        // Arrange
        let json = r#"{"status":"success","response":{"value":42}}"#;

        // Act
        let result: Result<ApiResponse<serde_json::Value>, _> = deserialize(json.to_owned());

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, "success");
        assert!(response.response.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn deserialize_failure_response() {
        // Arrange
        let json = r#"{"status":"failure","error":"bad id parameter"}"#;

        // Act
        let result: Result<ApiResponse<serde_json::Value>, _> = deserialize(json.to_owned());

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, "failure");
        assert!(response.response.is_none());
        assert_eq!(response.error, Some("bad id parameter".to_owned()));
    }

    #[test]
    fn deserialize_removes_malformed_ops_response() {
        // Arrange - OPS returns malformed "response":[] on errors
        let json = r#"{"status":"failure","response":[],"error":"bad id parameter"}"#;

        // Act
        let result: Result<ApiResponse<serde_json::Value>, _> = deserialize(json.to_owned());

        // Assert - should parse after removing malformed array
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, "failure");
        assert_eq!(response.error, Some("bad id parameter".to_owned()));
    }

    #[test]
    fn deserialize_invalid_json_returns_error() {
        // Arrange
        let json = r#"{"invalid json"#;

        // Act
        let result: Result<ApiResponse<serde_json::Value>, _> = deserialize(json.to_owned());

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Deserialization { .. }));
    }

    // get_result tests

    #[test]
    fn get_result_success_extracts_response() {
        // Arrange
        let response = ApiResponse {
            status: "success".to_owned(),
            response: Some(42),
            error: None,
        };

        // Act
        let result = get_result(StatusCode::OK, response);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn get_result_with_response_error_returns_error() {
        // Arrange
        let response: ApiResponse<i32> = ApiResponse {
            status: "failure".to_owned(),
            response: None,
            error: Some("bad id parameter".to_owned()),
        };

        // Act
        let result = get_result(StatusCode::OK, response);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BadRequest { .. }));
    }

    #[test]
    fn get_result_with_status_error_returns_error() {
        // Arrange - RED returns proper status codes
        let response: ApiResponse<i32> = ApiResponse {
            status: "failure".to_owned(),
            response: None,
            error: Some("unknown error".to_owned()),
        };

        // Act
        let result = get_result(StatusCode::BAD_REQUEST, response);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BadRequest { .. }));
    }

    #[test]
    fn get_result_no_response_returns_other_error() {
        // Arrange - Success status but no response body
        let response: ApiResponse<i32> = ApiResponse {
            status: "success".to_owned(),
            response: None,
            error: None,
        };

        // Act
        let result = get_result(StatusCode::OK, response);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Other { status: 200, .. }));
    }

    #[test]
    fn get_result_response_error_takes_priority_over_status() {
        // Arrange - Both response error and bad status
        let response: ApiResponse<i32> = ApiResponse {
            status: "failure".to_owned(),
            response: None,
            error: Some("Rate limit exceeded".to_owned()),
        };

        // Act - Status is 400 but error message indicates rate limit
        let result = get_result(StatusCode::BAD_REQUEST, response);

        // Assert - Response error matching takes priority
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TooManyRequests { .. }));
    }

    #[test]
    fn get_result_unknown_response_error_falls_through() {
        // Arrange
        let response: ApiResponse<i32> = ApiResponse {
            status: "failure".to_owned(),
            response: None,
            error: Some("some new error type".to_owned()),
        };

        // Act
        let result = get_result(StatusCode::OK, response);

        // Assert - Falls through to other error
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Other {
                status: 200,
                message: Some(msg)
            } if msg == "some new error type"
        ));
    }
}
