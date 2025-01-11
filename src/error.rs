use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use GazelleError::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(clippy::absolute_paths)]
pub enum GazelleError {
    /// An error occured creating the request
    /// Includes the `reqwest::Error` as a string
    Request(String),
    /// An error occured extracting the body of the response
    /// Includes the `reqwest::Error` as a string
    Response(String),
    /// An error occured deserializing the body as JSON
    /// Includes the `serde_json::Error` as a string
    Deserialization(String),
    /// An error occured reading the torrent file
    /// Includes the `std::io::Error` as a string
    Upload(String),
    /// 400 Bad Request
    /// Indicates that either the requested resource was not found,
    /// or there was an issue with the paramters
    BadRequest,
    /// 401 Unauthorized
    /// Indicates the API Key is invalid
    Unauthorized,
    /// 404 Not Found
    /// Indicates the requested resource was not found
    NotFound,
    /// 429 Too Many Request
    /// Indicates the rate limit has been hit
    TooManyRequests,
    /// An unexpected status code and error message was received from the API
    /// Includes the `StatusCode` as a `u16` and
    /// the error message received from the API as a string
    Unexpected(u16, String),
    /// An unexpected status code was received from the API and it did not contain an error message
    /// Includes the `StatusCode` as a `u16`
    Empty(u16),
}

#[allow(clippy::absolute_paths)]
impl GazelleError {
    pub(crate) fn request(error: reqwest::Error) -> Self {
        Request(error.to_string())
    }

    pub(crate) fn response(error: reqwest::Error) -> Self {
        Response(error.to_string())
    }

    pub(crate) fn deserialization(error: serde_json::Error) -> Self {
        Request(error.to_string())
    }

    pub(crate) fn upload(error: std::io::Error) -> Self {
        Upload(error.to_string())
    }

    pub(crate) fn unexpected(status_code: StatusCode, error: String) -> Self {
        Unexpected(status_code.as_u16(), error)
    }

    pub(crate) fn empty(status_code: StatusCode) -> Self {
        Empty(status_code.as_u16())
    }

    /// Get a `GazelleError` if the status code indicates a known client error
    /// *RED only as OPS inexplicably returns `200 Success` for everything*
    pub(crate) fn match_client_error(status_code: StatusCode) -> Option<Self> {
        match status_code {
            StatusCode::BAD_REQUEST => Some(BadRequest),
            StatusCode::UNAUTHORIZED => Some(Unauthorized),
            StatusCode::NOT_FOUND => Some(NotFound),
            StatusCode::TOO_MANY_REQUESTS => Some(TooManyRequests),
            _ => None,
        }
    }

    /// Get a `GazelleError` if the response `error` string indicates a known client error
    pub(crate) fn match_response_error(value: &str) -> Option<Self> {
        match value {
            "bad id parameter" | "bad parameters" => Some(BadRequest),
            "This page is limited to API key usage only." | "This page requires an api token" => {
                Some(Unauthorized)
            }
            "endpoint not found" | "failure" => Some(NotFound),
            "Rate limit exceeded" => Some(TooManyRequests),
            _ => None,
        }
    }
}

impl Display for GazelleError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Request(message) => format!("Failed to send API request: {message}"),
            Response(message) => {
                format!("Failed to read API response: {message}")
            }
            Deserialization(message) => {
                format!("Failed to deserialize API response: {message}")
            }
            Upload(message) => {
                format!("Failed to upload torrent file: {message}")
            }
            BadRequest => "Invalid parameters".to_owned(),
            Unauthorized => "Invalid API key".to_owned(),
            NotFound => "Resource does not exist".to_owned(),
            TooManyRequests => "Exceeded rate limit".to_owned(),
            Unexpected(code, message) => {
                format!(
                    "Unexpected API response ({}): {message}",
                    status_code_and_reason(*code)
                )
            }
            Empty(code) => format!(
                "Unexpected API response without error message ({})",
                status_code_and_reason(*code)
            ),
        };
        message.fmt(formatter)
    }
}

fn status_code_and_reason(code: u16) -> String {
    StatusCode::from_u16(code)
        .ok()
        .and_then(|code| code.canonical_reason())
        .map(|reason| format!("{code} {reason}"))
        .unwrap_or(code.to_string())
}
