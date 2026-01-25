use GazelleError::*;
use colored::Colorize;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::absolute_paths)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum GazelleError {
    /// An error occured creating the request
    /// Includes the `reqwest::Error` as a string
    Request { error: String },
    /// An error occured extracting the body of the response
    /// Includes the `reqwest::Error` as a string
    Response { error: String },
    /// An error occured deserializing the body as JSON
    /// Includes the `serde_json::Error` as a string
    Deserialization { error: String },
    /// An error occured reading the torrent file
    /// Includes the `std::io::Error` as a string
    Upload { error: String },
    /// 400 Bad Request
    /// Indicates that either the requested resource was not found,
    /// or there was an issue with the paramters
    BadRequest { message: String },
    /// 401 Unauthorized
    /// Indicates the API Key is invalid
    Unauthorized { message: String },
    /// 404 Not Found
    /// Indicates the requested resource was not found
    NotFound { message: String },
    /// 429 Too Many Request
    /// Indicates the rate limit has been hit
    TooManyRequests { message: String },
    /// An unexpected status code and error message was received from the API
    /// Includes the `StatusCode` as a `u16` and
    /// the error message received from the API as a string
    Other {
        status: u16,
        message: Option<String>,
    },
}

#[allow(clippy::absolute_paths)]
impl GazelleError {
    pub(crate) fn request(error: reqwest::Error) -> Self {
        Request {
            error: error.to_string(),
        }
    }

    pub(crate) fn response(error: reqwest::Error) -> Self {
        Response {
            error: error.to_string(),
        }
    }

    pub(crate) fn deserialization(error: serde_json::Error) -> Self {
        Request {
            error: error.to_string(),
        }
    }

    pub(crate) fn upload(error: std::io::Error) -> Self {
        Upload {
            error: error.to_string(),
        }
    }

    pub(crate) fn other(status_code: StatusCode, error: Option<String>) -> Self {
        Other {
            status: status_code.as_u16(),
            message: error,
        }
    }

    /// Get a `GazelleError` if the status code indicates a known client error
    /// *RED only as OPS inexplicably returns `200 Success` for everything*
    pub(crate) fn match_status_error(
        status_code: StatusCode,
        error: Option<String>,
    ) -> Option<Self> {
        match status_code {
            StatusCode::BAD_REQUEST => Some(BadRequest {
                message: error.unwrap_or_default(),
            }),
            StatusCode::UNAUTHORIZED => Some(Unauthorized {
                message: error.unwrap_or_default(),
            }),
            StatusCode::NOT_FOUND => Some(NotFound {
                message: error.unwrap_or_default(),
            }),
            StatusCode::TOO_MANY_REQUESTS => Some(TooManyRequests {
                message: error.unwrap_or_default(),
            }),
            _ => None,
        }
    }

    /// Get a `GazelleError` if the response `error` string indicates a known client error
    pub(crate) fn match_response_error(error: &str) -> Option<Self> {
        match error {
            "bad id parameter" | "bad parameters" | "no such user" => Some(BadRequest {
                message: error.to_owned(),
            }),
            "This page is limited to API key usage only." | "This page requires an api token" => {
                Some(Unauthorized {
                    message: error.to_owned(),
                })
            }
            "endpoint not found" | "failure" | "could not find torrent" => Some(NotFound {
                message: error.to_owned(),
            }),
            "Rate limit exceeded" => Some(TooManyRequests {
                message: error.to_owned(),
            }),
            _ => None,
        }
    }
}

impl Display for GazelleError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Request { error } => format!("{} to send API request: {error}", "Failed".bold()),
            Response { error } => {
                format!("{} to read API response: {error}", "Failed".bold())
            }
            Deserialization { error } => {
                format!("{} to deserialize API response: {error}", "Failed".bold())
            }
            Upload { error } => {
                format!("{} to upload torrent file: {error}", "Failed".bold())
            }
            BadRequest { message } => {
                format!(
                    "{} bad request response{}",
                    "Received".bold(),
                    append(message)
                )
            }
            Unauthorized { message } => {
                format!(
                    "{} unauthorized response{}",
                    "Received".bold(),
                    append(message)
                )
            }
            NotFound { message } => {
                format!(
                    "{} not found response{}",
                    "Received".bold(),
                    append(message)
                )
            }
            TooManyRequests { message } => {
                format!(
                    "{} too many requests response{}",
                    "Received".bold(),
                    append(message)
                )
            }
            Other {
                status,
                message: error,
            } => {
                format!(
                    "{} {} response{}",
                    "Received".bold(),
                    status_code_and_reason(*status),
                    append(&error.clone().unwrap_or_default())
                )
            }
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

fn append(message: &str) -> String {
    if message.is_empty() {
        String::new()
    } else {
        format!(": {message}")
    }
}

#[cfg(test)]
mod tests {
    use crate::GazelleError;
    use crate::GazelleError::*;
    use reqwest::StatusCode;

    #[test]
    pub fn yaml_serialization() -> Result<(), serde_yaml::Error> {
        // Arrange
        let example = vec![
            BadRequest {
                message: String::new(),
            },
            BadRequest {
                message: "bad id parameter".to_owned(),
            },
            NotFound {
                message: "no such user".to_owned(),
            },
            Other {
                status: 500,
                message: Some("Hello, world".to_owned()),
            },
        ];
        let expected = "- type: bad_request
  message: ''
- type: bad_request
  message: bad id parameter
- type: not_found
  message: no such user
- type: other
  status: 500
  message: Hello, world
";

        // Act
        let yaml = serde_yaml::to_string(&example)?;
        println!("{yaml}");
        let deserialized: Vec<GazelleError> = serde_yaml::from_str(expected)?;

        // Assert
        assert_eq!(yaml, expected);
        assert_eq!(deserialized, example);
        Ok(())
    }

    // match_status_error tests

    #[test]
    fn match_status_error_bad_request() {
        // Arrange & Act
        let result =
            GazelleError::match_status_error(StatusCode::BAD_REQUEST, Some("test".to_owned()));

        // Assert
        assert!(matches!(result, Some(BadRequest { message }) if message == "test"));
    }

    #[test]
    fn match_status_error_unauthorized() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::UNAUTHORIZED, None);

        // Assert
        assert!(matches!(result, Some(Unauthorized { message }) if message.is_empty()));
    }

    #[test]
    fn match_status_error_not_found() {
        // Arrange & Act
        let result =
            GazelleError::match_status_error(StatusCode::NOT_FOUND, Some("not found".to_owned()));

        // Assert
        assert!(matches!(result, Some(NotFound { message }) if message == "not found"));
    }

    #[test]
    fn match_status_error_too_many_requests() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::TOO_MANY_REQUESTS, None);

        // Assert
        assert!(matches!(result, Some(TooManyRequests { .. })));
    }

    #[test]
    fn match_status_error_success_returns_none() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::OK, None);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn match_status_error_server_error_returns_none() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::INTERNAL_SERVER_ERROR, None);

        // Assert
        assert!(result.is_none());
    }

    // match_response_error tests

    #[test]
    fn match_response_error_bad_id() {
        // Arrange & Act
        let result = GazelleError::match_response_error("bad id parameter");

        // Assert
        assert!(matches!(result, Some(BadRequest { .. })));
    }

    #[test]
    fn match_response_error_bad_parameters() {
        // Arrange & Act
        let result = GazelleError::match_response_error("bad parameters");

        // Assert
        assert!(matches!(result, Some(BadRequest { .. })));
    }

    #[test]
    fn match_response_error_no_such_user() {
        // Arrange & Act
        let result = GazelleError::match_response_error("no such user");

        // Assert
        assert!(matches!(result, Some(BadRequest { .. })));
    }

    #[test]
    fn match_response_error_api_key_only() {
        // Arrange & Act
        let result =
            GazelleError::match_response_error("This page is limited to API key usage only.");

        // Assert
        assert!(matches!(result, Some(Unauthorized { .. })));
    }

    #[test]
    fn match_response_error_api_token_required() {
        // Arrange & Act
        let result = GazelleError::match_response_error("This page requires an api token");

        // Assert
        assert!(matches!(result, Some(Unauthorized { .. })));
    }

    #[test]
    fn match_response_error_endpoint_not_found() {
        // Arrange & Act
        let result = GazelleError::match_response_error("endpoint not found");

        // Assert
        assert!(matches!(result, Some(NotFound { .. })));
    }

    #[test]
    fn match_response_error_failure() {
        // Arrange & Act
        let result = GazelleError::match_response_error("failure");

        // Assert
        assert!(matches!(result, Some(NotFound { .. })));
    }

    #[test]
    fn match_response_error_could_not_find_torrent() {
        // Arrange & Act
        let result = GazelleError::match_response_error("could not find torrent");

        // Assert
        assert!(matches!(result, Some(NotFound { .. })));
    }

    #[test]
    fn match_response_error_rate_limit() {
        // Arrange & Act
        let result = GazelleError::match_response_error("Rate limit exceeded");

        // Assert
        assert!(matches!(result, Some(TooManyRequests { .. })));
    }

    #[test]
    fn match_response_error_unknown_returns_none() {
        // Arrange & Act
        let result = GazelleError::match_response_error("some unknown error message");

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn match_response_error_empty_returns_none() {
        // Arrange & Act
        let result = GazelleError::match_response_error("");

        // Assert
        assert!(result.is_none());
    }
}
