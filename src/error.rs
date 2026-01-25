use colored::Colorize;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io;

/// Error kind without message payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GazelleErrorKind {
    /// Failed to send HTTP request
    Request,
    /// Failed to read HTTP response body
    Response,
    /// Failed to deserialize JSON
    Deserialization,
    /// Failed to read torrent file for upload
    Upload,
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 404 Not Found
    NotFound,
    /// 429 Too Many Requests
    TooManyRequests,
    /// Other HTTP status code
    Other(u16),
}

/// Unified error type with source chain preservation.
#[derive(Serialize, Deserialize)]
pub struct GazelleError {
    pub kind: GazelleErrorKind,
    pub message: Option<String>,
    #[serde(skip)]
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl GazelleError {
    /// Create error without source
    #[must_use]
    pub fn new(kind: GazelleErrorKind) -> Self {
        Self {
            kind,
            message: None,
            source: None,
        }
    }

    /// Create error with message
    #[must_use]
    pub fn with_message(kind: GazelleErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: Some(message.into()),
            source: None,
        }
    }

    /// Create error with source
    pub fn with_source<E: Error + Send + Sync + 'static>(
        kind: GazelleErrorKind,
        source: E,
    ) -> Self {
        Self {
            kind,
            message: None,
            source: Some(Box::new(source)),
        }
    }

    /// Clone the error without the source chain.
    /// Use this when you need a copy but don't need error chain propagation.
    #[must_use]
    pub fn clone_without_source(&self) -> Self {
        Self {
            kind: self.kind,
            message: self.message.clone(),
            source: None,
        }
    }

    /// Create a request error from a reqwest error
    pub(crate) fn request(source: reqwest::Error) -> Self {
        Self::with_source(GazelleErrorKind::Request, source)
    }

    /// Create a response error from a reqwest error
    pub(crate) fn response(source: reqwest::Error) -> Self {
        Self::with_source(GazelleErrorKind::Response, source)
    }

    /// Create a deserialization error from a `serde_json` error
    pub(crate) fn deserialization(source: serde_json::Error) -> Self {
        Self::with_source(GazelleErrorKind::Deserialization, source)
    }

    /// Create an upload error from an io error
    pub(crate) fn upload(source: io::Error) -> Self {
        Self::with_source(GazelleErrorKind::Upload, source)
    }

    /// Create an other error from a status code
    pub(crate) fn other(status: StatusCode, message: Option<String>) -> Self {
        Self {
            kind: GazelleErrorKind::Other(status.as_u16()),
            message,
            source: None,
        }
    }

    /// Match HTTP status code to error kind
    /// *RED only as OPS inexplicably returns `200 Success` for everything*
    pub(crate) fn match_status_error(status: StatusCode, message: Option<String>) -> Option<Self> {
        let kind = match status {
            StatusCode::BAD_REQUEST => GazelleErrorKind::BadRequest,
            StatusCode::UNAUTHORIZED => GazelleErrorKind::Unauthorized,
            StatusCode::NOT_FOUND => GazelleErrorKind::NotFound,
            StatusCode::TOO_MANY_REQUESTS => GazelleErrorKind::TooManyRequests,
            _ => return None,
        };
        Some(Self {
            kind,
            message,
            source: None,
        })
    }

    /// Match API error string to error kind
    pub(crate) fn match_response_error(error: &str) -> Option<Self> {
        let kind = match error {
            "bad id parameter" | "bad parameters" | "no such user" => GazelleErrorKind::BadRequest,
            "This page is limited to API key usage only." | "This page requires an api token" => {
                GazelleErrorKind::Unauthorized
            }
            "endpoint not found" | "failure" | "could not find torrent" => {
                GazelleErrorKind::NotFound
            }
            "Rate limit exceeded" => GazelleErrorKind::TooManyRequests,
            _ => return None,
        };
        Some(Self::with_message(kind, error))
    }
}

impl Error for GazelleError {
    #[allow(clippy::as_conversions)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

impl Display for GazelleErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            GazelleErrorKind::Request => write!(f, "{} to send API request", "Failed".bold()),
            GazelleErrorKind::Response => write!(f, "{} to read API response", "Failed".bold()),
            GazelleErrorKind::Deserialization => {
                write!(f, "{} to deserialize API response", "Failed".bold())
            }
            GazelleErrorKind::Upload => {
                write!(f, "{} to upload torrent file", "Failed".bold())
            }
            GazelleErrorKind::BadRequest => {
                write!(f, "{} bad request response", "Received".bold())
            }
            GazelleErrorKind::Unauthorized => {
                write!(f, "{} unauthorized response", "Received".bold())
            }
            GazelleErrorKind::NotFound => write!(f, "{} not found response", "Received".bold()),
            GazelleErrorKind::TooManyRequests => {
                write!(f, "{} too many requests response", "Received".bold())
            }
            GazelleErrorKind::Other(status) => {
                write!(
                    f,
                    "{} {} response",
                    "Received".bold(),
                    status_code_and_reason(*status)
                )
            }
        }
    }
}

impl Display for GazelleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.kind)?;
        if let Some(msg) = &self.message {
            write!(f, "\n{msg}")?;
        }
        if let Some(source) = &self.source {
            write!(f, "\n{source}")?;
        }
        Ok(())
    }
}

impl Debug for GazelleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debug = f.debug_struct("GazelleError");
        debug.field("kind", &self.kind);
        if let Some(msg) = &self.message {
            debug.field("message", msg);
        }
        if let Some(src) = &self.source {
            debug.field("source", src);
        }
        debug.finish()
    }
}

fn status_code_and_reason(code: u16) -> String {
    StatusCode::from_u16(code)
        .ok()
        .and_then(|code| code.canonical_reason())
        .map(|reason| format!("{code} {reason}"))
        .unwrap_or(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_serialization_all_kinds() {
        // Arrange
        let errors = vec![
            GazelleError::with_message(GazelleErrorKind::Request, "connection refused"),
            GazelleError::with_message(GazelleErrorKind::Response, "invalid body"),
            GazelleError::with_message(GazelleErrorKind::Deserialization, "expected string"),
            GazelleError::with_message(GazelleErrorKind::Upload, "file not found"),
            GazelleError::with_message(GazelleErrorKind::BadRequest, "bad id parameter"),
            GazelleError::with_message(GazelleErrorKind::Unauthorized, "invalid api key"),
            GazelleError::with_message(GazelleErrorKind::NotFound, "torrent not found"),
            GazelleError::with_message(GazelleErrorKind::TooManyRequests, "rate limit exceeded"),
            GazelleError {
                kind: GazelleErrorKind::Other(500),
                message: Some("internal error".to_owned()),
                source: None,
            },
            GazelleError::new(GazelleErrorKind::Other(503)),
        ];

        // Act
        let yaml = serde_yaml::to_string(&errors).expect("should serialize");

        // Assert
        insta::assert_snapshot!(yaml);

        // Act (round-trip)
        let deserialized: Vec<GazelleError> =
            serde_yaml::from_str(&yaml).expect("should deserialize");

        // Assert
        assert_eq!(deserialized.len(), errors.len());
        for (orig, deser) in errors.iter().zip(deserialized.iter()) {
            assert_eq!(orig.kind, deser.kind);
            assert_eq!(orig.message, deser.message);
        }
    }

    #[test]
    fn source_is_preserved() {
        // Arrange
        let json_err = serde_json::from_str::<()>("invalid").expect_err("should fail");

        // Act
        let error = GazelleError::deserialization(json_err);

        // Assert
        assert!(error.source().is_some());
        assert_eq!(error.kind, GazelleErrorKind::Deserialization);
    }

    #[test]
    fn new_has_no_source() {
        // Act
        let error = GazelleError::new(GazelleErrorKind::NotFound);

        // Assert
        assert!(error.source().is_none());
        assert!(error.message.is_none());
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
    }

    #[test]
    fn with_message_has_no_source() {
        // Act
        let error = GazelleError::with_message(GazelleErrorKind::NotFound, "test message");

        // Assert
        assert!(error.source().is_none());
        assert_eq!(error.message, Some("test message".to_owned()));
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
    }

    // match_status_error tests

    #[test]
    fn match_status_error_bad_request() {
        // Arrange & Act
        let result =
            GazelleError::match_status_error(StatusCode::BAD_REQUEST, Some("test".to_owned()));

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::BadRequest);
        assert_eq!(error.message, Some("test".to_owned()));
    }

    #[test]
    fn match_status_error_unauthorized() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::UNAUTHORIZED, None);

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::Unauthorized);
        assert!(error.message.is_none());
    }

    #[test]
    fn match_status_error_not_found() {
        // Arrange & Act
        let result =
            GazelleError::match_status_error(StatusCode::NOT_FOUND, Some("not found".to_owned()));

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
        assert_eq!(error.message, Some("not found".to_owned()));
    }

    #[test]
    fn match_status_error_too_many_requests() {
        // Arrange & Act
        let result = GazelleError::match_status_error(StatusCode::TOO_MANY_REQUESTS, None);

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::TooManyRequests);
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
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::BadRequest);
    }

    #[test]
    fn match_response_error_bad_parameters() {
        // Arrange & Act
        let result = GazelleError::match_response_error("bad parameters");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::BadRequest);
    }

    #[test]
    fn match_response_error_no_such_user() {
        // Arrange & Act
        let result = GazelleError::match_response_error("no such user");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::BadRequest);
    }

    #[test]
    fn match_response_error_api_key_only() {
        // Arrange & Act
        let result =
            GazelleError::match_response_error("This page is limited to API key usage only.");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::Unauthorized);
    }

    #[test]
    fn match_response_error_api_token_required() {
        // Arrange & Act
        let result = GazelleError::match_response_error("This page requires an api token");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::Unauthorized);
    }

    #[test]
    fn match_response_error_endpoint_not_found() {
        // Arrange & Act
        let result = GazelleError::match_response_error("endpoint not found");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
    }

    #[test]
    fn match_response_error_failure() {
        // Arrange & Act
        let result = GazelleError::match_response_error("failure");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
    }

    #[test]
    fn match_response_error_could_not_find_torrent() {
        // Arrange & Act
        let result = GazelleError::match_response_error("could not find torrent");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::NotFound);
    }

    #[test]
    fn match_response_error_rate_limit() {
        // Arrange & Act
        let result = GazelleError::match_response_error("Rate limit exceeded");

        // Assert
        let error = result.expect("should match");
        assert_eq!(error.kind, GazelleErrorKind::TooManyRequests);
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
