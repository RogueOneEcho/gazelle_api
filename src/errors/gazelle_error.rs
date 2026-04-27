use crate::prelude::*;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to {}", self.operation)
    }
}

impl Error for GazelleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
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

impl GazelleError {
    pub(crate) fn request(source: ReqwestError) -> Self {
        Self {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Reqwest(source),
        }
    }

    pub(crate) fn response(source: ReqwestError) -> Self {
        Self {
            operation: GazelleOperation::ReadResponse,
            source: ErrorSource::Reqwest(source),
        }
    }

    pub(crate) fn deserialization(source: JsonError) -> Self {
        Self {
            operation: GazelleOperation::Deserialize,
            source: ErrorSource::SerdeJson(source),
        }
    }

    pub(crate) fn upload(source: IoError) -> Self {
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

    /// Whether the error is a transient rate-limit that warrants retry.
    ///
    /// - Returns `false` for non-API-response operations
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self.operation {
            GazelleOperation::ApiResponse(kind) => kind.is_retryable(),
            _ => false,
        }
    }

    /// Whether the error indicates the requested resource is missing.
    ///
    /// - Returns `false` for non-API-response operations
    #[must_use]
    pub fn is_missing(&self) -> bool {
        match self.operation {
            GazelleOperation::ApiResponse(kind) => kind.is_missing(),
            _ => false,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_status_error_bad_request() {
        let result =
            GazelleError::match_status_error(StatusCode::BAD_REQUEST, Some("test".to_owned()));
        assert!(result.is_some());
        let error = result.expect("bad request should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_status_error_unauthorized() {
        let result = GazelleError::match_status_error(StatusCode::UNAUTHORIZED, None);
        assert!(result.is_some());
        let error = result.expect("unauthorized should match");
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
        let error = result.expect("not found should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_status_error_too_many_requests() {
        let result = GazelleError::match_status_error(StatusCode::TOO_MANY_REQUESTS, None);
        assert!(result.is_some());
        let error = result.expect("too many requests should match");
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
        let error = result.expect("bad id should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_response_error_bad_parameters() {
        let result = GazelleError::match_response_error("bad parameters", 200);
        assert!(result.is_some());
        let error = result.expect("bad parameters should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::BadRequest)
        );
    }

    #[test]
    fn match_response_error_no_such_user() {
        let result = GazelleError::match_response_error("no such user", 200);
        assert!(result.is_some());
        let error = result.expect("no such user should match");
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
        let error = result.expect("api key only should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::Unauthorized)
        );
    }

    #[test]
    fn match_response_error_api_token_required() {
        let result = GazelleError::match_response_error("This page requires an api token", 200);
        assert!(result.is_some());
        let error = result.expect("api token required should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::Unauthorized)
        );
    }

    #[test]
    fn match_response_error_endpoint_not_found() {
        let result = GazelleError::match_response_error("endpoint not found", 200);
        assert!(result.is_some());
        let error = result.expect("endpoint not found should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_failure() {
        let result = GazelleError::match_response_error("failure", 200);
        assert!(result.is_some());
        let error = result.expect("failure should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_could_not_find_torrent() {
        let result = GazelleError::match_response_error("could not find torrent", 200);
        assert!(result.is_some());
        let error = result.expect("could not find torrent should match");
        assert_eq!(
            error.operation,
            GazelleOperation::ApiResponse(ApiResponseKind::NotFound)
        );
    }

    #[test]
    fn match_response_error_rate_limit() {
        let result = GazelleError::match_response_error("Rate limit exceeded", 200);
        assert!(result.is_some());
        let error = result.expect("rate limit should match");
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
    fn is_retryable_too_many_requests() {
        let error = GazelleError::too_many_requests("Rate limit exceeded".to_owned(), 429);
        assert!(error.is_retryable());
    }

    #[test]
    fn is_retryable_bad_request() {
        let error = GazelleError::bad_request("bad id parameter".to_owned(), 400);
        assert!(!error.is_retryable());
    }

    #[test]
    fn is_retryable_unauthorized() {
        let error = GazelleError::unauthorized("nope".to_owned(), 401);
        assert!(!error.is_retryable());
    }

    #[test]
    fn is_retryable_not_found() {
        let error = GazelleError::not_found("nope".to_owned(), 404);
        assert!(!error.is_retryable());
    }

    #[test]
    fn is_retryable_other() {
        let error = GazelleError::other("boom".to_owned(), 500);
        assert!(!error.is_retryable());
    }

    #[test]
    fn is_retryable_send_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(IoError::other("network down")),
        };
        assert!(!error.is_retryable());
    }

    #[test]
    fn is_missing_not_found() {
        let error = GazelleError::not_found("nope".to_owned(), 404);
        assert!(error.is_missing());
    }

    #[test]
    fn is_missing_bad_request() {
        let error = GazelleError::bad_request("bad id parameter".to_owned(), 400);
        assert!(error.is_missing());
    }

    #[test]
    fn is_missing_unauthorized() {
        let error = GazelleError::unauthorized("nope".to_owned(), 401);
        assert!(!error.is_missing());
    }

    #[test]
    fn is_missing_too_many_requests() {
        let error = GazelleError::too_many_requests("Rate limit exceeded".to_owned(), 429);
        assert!(!error.is_missing());
    }

    #[test]
    fn is_missing_other() {
        let error = GazelleError::other("boom".to_owned(), 500);
        assert!(!error.is_missing());
    }

    #[test]
    fn is_missing_send_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(IoError::other("network down")),
        };
        assert!(!error.is_missing());
    }

    #[test]
    fn diagnostic_code_send_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(IoError::other("test")),
        };
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::SendRequest");
    }

    #[test]
    fn diagnostic_code_read_response() {
        let error = GazelleError {
            operation: GazelleOperation::ReadResponse,
            source: ErrorSource::Io(IoError::other("test")),
        };
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ReadResponse");
    }

    #[test]
    fn diagnostic_code_deserialize() {
        let error = GazelleError::deserialization(
            json_from_str::<()>("invalid").expect_err("invalid json should fail"),
        );
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::Deserialize");
    }

    #[test]
    fn diagnostic_code_read_file() {
        let error = GazelleError::upload(IoError::other("test"));
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ReadFile");
    }

    #[test]
    fn diagnostic_code_api_response_not_found() {
        let error = GazelleError::not_found("test".to_owned(), 404);
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(NotFound)");
    }

    #[test]
    fn diagnostic_code_api_response_unauthorized() {
        let error = GazelleError::unauthorized("test".to_owned(), 401);
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(Unauthorized)");
    }

    #[test]
    fn diagnostic_code_api_response_bad_request() {
        let error = GazelleError::bad_request("test".to_owned(), 400);
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(BadRequest)");
    }

    #[test]
    fn diagnostic_code_api_response_too_many_requests() {
        let error = GazelleError::too_many_requests("test".to_owned(), 429);
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(TooManyRequests)");
    }

    #[test]
    fn diagnostic_code_api_response_other() {
        let error = GazelleError::other("I'm a teapot".to_owned(), 418);
        let code = error.code().expect("should have code").to_string();
        assert_eq!(code, "gazelle_api::ApiResponse(Other)");
    }
}
