use crate::GazelleSerializableError::*;
use miette::Diagnostic;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use thiserror::Error as ThisError;

/// The kind of API response error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiResponseKind {
    BadRequest,
    Unauthorized,
    NotFound,
    TooManyRequests,
    Other,
}

impl Display for ApiResponseKind {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest => write!(f, "bad request"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::NotFound => write!(f, "not found"),
            Self::TooManyRequests => write!(f, "too many requests"),
            Self::Other => write!(f, "unexpected response"),
        }
    }
}

/// The operation that failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ThisError, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "kind")]
pub enum GazelleOperation {
    #[error("send request")]
    SendRequest,
    #[error("read response body")]
    ReadResponse,
    #[error("deserialize response")]
    Deserialize,
    #[error("read file")]
    ReadFile,
    #[error("{0}")]
    ApiResponse(ApiResponseKind),
}

/// An error from the API response.
#[derive(Clone, Debug, ThisError)]
#[error("{message}")]
pub struct ApiResponseError {
    pub message: String,
    pub status: u16,
}

/// The source of a [`GazelleError`].
#[derive(Debug)]
pub enum ErrorSource {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Io(io::Error),
    ApiResponse(ApiResponseError),
    Stringified(String),
}

impl Display for ErrorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "{e}"),
            Self::SerdeJson(e) => write!(f, "{e}"),
            Self::Io(e) => write!(f, "{e}"),
            Self::ApiResponse(e) => write!(f, "{e}"),
            Self::Stringified(s) => write!(f, "{s}"),
        }
    }
}

#[cfg(feature = "mock")]
impl Clone for ErrorSource {
    fn clone(&self) -> Self {
        match self {
            Self::ApiResponse(e) => Self::ApiResponse(e.clone()),
            other => Self::Stringified(other.to_string()),
        }
    }
}

#[cfg(feature = "mock")]
impl Clone for GazelleError {
    fn clone(&self) -> Self {
        Self {
            operation: self.operation,
            source: self.source.clone(),
        }
    }
}

impl Error for ErrorSource {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Reqwest(e) => Some(e),
            Self::SerdeJson(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::ApiResponse(e) => Some(e),
            Self::Stringified(_) => None,
        }
    }
}

/// A structured error from the Gazelle API.
#[derive(Debug)]
pub struct GazelleError {
    pub operation: GazelleOperation,
    pub source: ErrorSource,
}

impl Diagnostic for GazelleError {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!(
            "{}::{:?}",
            env!("CARGO_PKG_NAME"),
            self.operation
        )))
    }
}

impl Display for GazelleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to {}", self.operation)
    }
}

impl Error for GazelleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl GazelleError {
    pub(crate) fn request(source: reqwest::Error) -> Self {
        Self {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Reqwest(source),
        }
    }

    pub(crate) fn response(source: reqwest::Error) -> Self {
        Self {
            operation: GazelleOperation::ReadResponse,
            source: ErrorSource::Reqwest(source),
        }
    }

    pub(crate) fn deserialization(source: serde_json::Error) -> Self {
        Self {
            operation: GazelleOperation::Deserialize,
            source: ErrorSource::SerdeJson(source),
        }
    }

    pub(crate) fn upload(source: io::Error) -> Self {
        Self {
            operation: GazelleOperation::ReadFile,
            source: ErrorSource::Io(source),
        }
    }

    pub(crate) fn api_response(kind: ApiResponseKind, message: String, status: u16) -> Self {
        Self {
            operation: GazelleOperation::ApiResponse(kind),
            source: ErrorSource::ApiResponse(ApiResponseError { message, status }),
        }
    }

    pub(crate) fn bad_request(message: String, status: u16) -> Self {
        Self::api_response(ApiResponseKind::BadRequest, message, status)
    }

    pub(crate) fn unauthorized(message: String, status: u16) -> Self {
        Self::api_response(ApiResponseKind::Unauthorized, message, status)
    }

    pub(crate) fn not_found(message: String, status: u16) -> Self {
        Self::api_response(ApiResponseKind::NotFound, message, status)
    }

    pub(crate) fn too_many_requests(message: String, status: u16) -> Self {
        Self::api_response(ApiResponseKind::TooManyRequests, message, status)
    }

    pub(crate) fn other(message: String, status: u16) -> Self {
        Self::api_response(ApiResponseKind::Other, message, status)
    }

    /// Get a [`GazelleError`] if the status code indicates a known client error.
    ///
    /// *RED only as OPS returns `200 Success` for everything*
    pub(crate) fn match_status_error(
        status_code: StatusCode,
        message: Option<String>,
    ) -> Option<Self> {
        let message = message.unwrap_or_default();
        let status = status_code.as_u16();
        match status_code {
            StatusCode::BAD_REQUEST => Some(Self::bad_request(message, status)),
            StatusCode::UNAUTHORIZED => Some(Self::unauthorized(message, status)),
            StatusCode::NOT_FOUND => Some(Self::not_found(message, status)),
            StatusCode::TOO_MANY_REQUESTS => Some(Self::too_many_requests(message, status)),
            _ => None,
        }
    }

    /// Get a [`GazelleError`] if the response error string indicates a known client error.
    pub(crate) fn match_response_error(error: &str, status: u16) -> Option<Self> {
        let message = error.to_owned();
        match error {
            "bad id parameter" | "bad parameters" | "no such user" => {
                Some(Self::bad_request(message, status))
            }
            "This page is limited to API key usage only." | "This page requires an api token" => {
                Some(Self::unauthorized(message, status))
            }
            "endpoint not found" | "failure" | "could not find torrent" => {
                Some(Self::not_found(message, status))
            }
            "Rate limit exceeded" => Some(Self::too_many_requests(message, status)),
            _ => None,
        }
    }
}

/// A serializable error from the Gazelle API.
///
/// This type preserves backwards compatibility with existing serialization formats.
/// Use [`From<GazelleError>`] to convert from the structured [`GazelleError`] type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum GazelleSerializableError {
    /// An error occurred creating the request.
    ///
    /// Includes the `reqwest::Error` as a string.
    Request { error: String },
    /// An error occurred extracting the body of the response.
    ///
    /// Includes the `reqwest::Error` as a string.
    Response { error: String },
    /// An error occurred deserializing the body as JSON.
    ///
    /// Includes the `serde_json::Error` as a string.
    Deserialization { error: String },
    /// An error occurred reading the torrent file.
    ///
    /// Includes the `std::io::Error` as a string.
    Upload { error: String },
    /// 400 Bad Request.
    ///
    /// Indicates that either the requested resource was not found,
    /// or there was an issue with the parameters.
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

impl From<GazelleError> for GazelleSerializableError {
    fn from(error: GazelleError) -> Self {
        match (error.operation, error.source) {
            (GazelleOperation::SendRequest, source) => Self::Request {
                error: source.to_string(),
            },
            (GazelleOperation::ReadResponse, source) => Self::Response {
                error: source.to_string(),
            },
            (GazelleOperation::Deserialize, source) => Self::Deserialization {
                error: source.to_string(),
            },
            (GazelleOperation::ReadFile, source) => Self::Upload {
                error: source.to_string(),
            },
            (GazelleOperation::ApiResponse(kind), ErrorSource::ApiResponse(api_err)) => {
                match kind {
                    ApiResponseKind::BadRequest => Self::BadRequest {
                        message: api_err.message,
                    },
                    ApiResponseKind::Unauthorized => Self::Unauthorized {
                        message: api_err.message,
                    },
                    ApiResponseKind::NotFound => Self::NotFound {
                        message: api_err.message,
                    },
                    ApiResponseKind::TooManyRequests => Self::TooManyRequests {
                        message: api_err.message,
                    },
                    ApiResponseKind::Other => Self::Other {
                        status: api_err.status,
                        message: Some(api_err.message),
                    },
                }
            }
            (GazelleOperation::ApiResponse(_), _) => {
                unreachable!("ApiResponse operation must have ApiResponse source")
            }
        }
    }
}

impl Display for GazelleSerializableError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Request { error } => format!("{} to send API request: {error}", "Failed"),
            Response { error } => {
                format!("{} to read API response: {error}", "Failed")
            }
            Deserialization { error } => {
                format!("{} to deserialize API response: {error}", "Failed")
            }
            Upload { error } => {
                format!("{} to upload torrent file: {error}", "Failed")
            }
            BadRequest { message } => {
                format!("{} bad request response{}", "Received", append(message))
            }
            Unauthorized { message } => {
                format!("{} unauthorized response{}", "Received", append(message))
            }
            NotFound { message } => {
                format!("{} not found response{}", "Received", append(message))
            }
            TooManyRequests { message } => {
                format!(
                    "{} too many requests response{}",
                    "Received",
                    append(message)
                )
            }
            Other {
                status,
                message: error,
            } => {
                format!(
                    "{} {} response{}",
                    "Received",
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
#[allow(clippy::unwrap_used, clippy::absolute_paths)]
mod tests {
    use super::*;

    #[test]
    fn yaml_serialization() -> Result<(), serde_yaml::Error> {
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
        let yaml = serde_yaml::to_string(&example)?;
        println!("{yaml}");
        let deserialized: Vec<GazelleSerializableError> = serde_yaml::from_str(expected)?;
        assert_eq!(yaml, expected);
        assert_eq!(deserialized, example);
        Ok(())
    }

    #[test]
    fn match_status_error_bad_request() {
        let result =
            GazelleError::match_status_error(StatusCode::BAD_REQUEST, Some("test".to_owned()));
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_status_error_unauthorized() {
        let result = GazelleError::match_status_error(StatusCode::UNAUTHORIZED, None);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::Unauthorized)
        );
    }

    #[test]
    fn match_status_error_not_found() {
        let result =
            GazelleError::match_status_error(StatusCode::NOT_FOUND, Some("not found".to_owned()));
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_status_error_too_many_requests() {
        let result = GazelleError::match_status_error(StatusCode::TOO_MANY_REQUESTS, None);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::TooManyRequests)
        );
    }

    #[test]
    fn match_status_error_success_returns_none() {
        let result = GazelleError::match_status_error(StatusCode::OK, None);
        assert!(result.is_none());
    }

    #[test]
    fn match_status_error_server_error_returns_none() {
        let result = GazelleError::match_status_error(StatusCode::INTERNAL_SERVER_ERROR, None);
        assert!(result.is_none());
    }

    #[test]
    fn match_response_error_bad_id() {
        let result = GazelleError::match_response_error("bad id parameter", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_response_error_bad_parameters() {
        let result = GazelleError::match_response_error("bad parameters", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_response_error_no_such_user() {
        let result = GazelleError::match_response_error("no such user", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_response_error_api_key_only() {
        let result =
            GazelleError::match_response_error("This page is limited to API key usage only.", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::Unauthorized)
        );
    }

    #[test]
    fn match_response_error_api_token_required() {
        let result = GazelleError::match_response_error("This page requires an api token", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::Unauthorized)
        );
    }

    #[test]
    fn match_response_error_endpoint_not_found() {
        let result = GazelleError::match_response_error("endpoint not found", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_failure() {
        let result = GazelleError::match_response_error("failure", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_could_not_find_torrent() {
        let result = GazelleError::match_response_error("could not find torrent", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_rate_limit() {
        let result = GazelleError::match_response_error("Rate limit exceeded", 200);
        assert!(result.is_some());
        let error = result.unwrap();
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::TooManyRequests)
        );
    }

    #[test]
    fn match_response_error_unknown_returns_none() {
        let result = GazelleError::match_response_error("some unknown error message", 200);
        assert!(result.is_none());
    }

    #[test]
    fn match_response_error_empty_returns_none() {
        let result = GazelleError::match_response_error("", 200);
        assert!(result.is_none());
    }

    #[test]
    fn conversion_to_serializable_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(std::io::Error::other("test")),
        };
        let serializable = GazelleSerializableError::from(error);
        assert!(
            matches!(serializable, GazelleSerializableError::Request { error } if error == "test")
        );
    }

    #[test]
    fn conversion_to_serializable_api_response() {
        let error = GazelleError::not_found("resource not found".to_owned(), 404);
        let serializable = GazelleSerializableError::from(error);
        assert!(
            matches!(serializable, GazelleSerializableError::NotFound { message } if message == "resource not found")
        );
    }

    #[test]
    fn conversion_to_serializable_other() {
        let error = GazelleError::other("unexpected".to_owned(), 500);
        let serializable = GazelleSerializableError::from(error);
        assert!(
            matches!(serializable, GazelleSerializableError::Other { status: 500, message: Some(m) } if m == "unexpected")
        );
    }

    #[test]
    fn diagnostic_code_send_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(std::io::Error::other("test")),
        };
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::SendRequest");
    }

    #[test]
    fn diagnostic_code_read_response() {
        let error = GazelleError {
            operation: GazelleOperation::ReadResponse,
            source: ErrorSource::Io(std::io::Error::other("test")),
        };
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ReadResponse");
    }

    #[test]
    fn diagnostic_code_deserialize() {
        let error =
            GazelleError::deserialization(serde_json::from_str::<()>("invalid").unwrap_err());
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::Deserialize");
    }

    #[test]
    fn diagnostic_code_read_file() {
        let error = GazelleError::upload(std::io::Error::other("test"));
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ReadFile");
    }

    #[test]
    fn diagnostic_code_api_response_not_found() {
        let error = GazelleError::not_found("test".to_owned(), 404);
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(NotFound)");
    }

    #[test]
    fn diagnostic_code_api_response_unauthorized() {
        let error = GazelleError::unauthorized("test".to_owned(), 401);
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(Unauthorized)");
    }

    #[test]
    fn diagnostic_code_api_response_bad_request() {
        let error = GazelleError::bad_request("test".to_owned(), 400);
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(BadRequest)");
    }

    #[test]
    fn diagnostic_code_api_response_too_many_requests() {
        let error = GazelleError::too_many_requests("test".to_owned(), 429);
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(TooManyRequests)");
    }

    #[test]
    fn diagnostic_code_api_response_other() {
        let error = GazelleError::other("I'm a teapot".to_owned(), 418);
        let code = error.code().unwrap().to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(Other)");
    }
}
