use reqwest::StatusCode;

#[derive(Debug)]
#[allow(clippy::absolute_paths)]
pub enum GazelleError {
    /// Error with request
    Request(reqwest::Error),
    /// Error with deserialization
    Deserialization(serde_json::Error, String),
    /// Error with I/O operation
    IO(std::io::Error),
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 404 Not Found
    NotFound,
    /// 429 Too Many Request
    TooManyRequests,
    /// Other
    Other(StatusCode, String),
}

impl GazelleError {
    pub(crate) fn from_status_code(status_code: StatusCode) -> Option<Self> {
        match status_code {
            StatusCode::BAD_REQUEST => Some(GazelleError::BadRequest),
            StatusCode::UNAUTHORIZED => Some(GazelleError::Unauthorized),
            StatusCode::NOT_FOUND => Some(GazelleError::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Some(GazelleError::TooManyRequests),
            _ => None,
        }
    }

    pub(crate) fn from_str(value: &str) -> Option<Self> {
        match value {
            "bad id parameter" | "bad parameters" => Some(GazelleError::BadRequest),
            "This page is limited to API key usage only." | "This page requires an api token" => {
                Some(GazelleError::Unauthorized)
            }
            "endpoint not found" | "failure" => Some(GazelleError::NotFound),
            "Rate limit exceeded" => Some(GazelleError::TooManyRequests),
            _ => None,
        }
    }
}
