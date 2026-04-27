use crate::GazelleSerializableError::*;
use crate::prelude::*;

/// A serializable error from the Gazelle API.
///
/// This type preserves backwards compatibility with existing serialization formats.
/// Use [`From<GazelleError>`] to convert from the structured [`GazelleError`] type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    /// Includes the `IoError` as a string.
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
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
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
mod tests {
    use super::*;

    #[test]
    fn yaml_serialization() -> Result<(), YamlError> {
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
        let yaml = yaml_to_string(&example)?;
        println!("{yaml}");
        let deserialized: Vec<GazelleSerializableError> = yaml_from_str(expected)?;
        assert_eq!(yaml, expected);
        assert_eq!(deserialized, example);
        Ok(())
    }

    #[test]
    fn conversion_to_serializable_request() {
        let error = GazelleError {
            operation: GazelleOperation::SendRequest,
            source: ErrorSource::Io(IoError::other("test")),
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
}
