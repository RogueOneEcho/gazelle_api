use crate::prelude::*;

/// The kind of API response error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiResponseKind {
    BadRequest,
    Unauthorized,
    NotFound,
    TooManyRequests,
    Other,
}

impl ApiResponseKind {
    /// Whether the response indicates a transient rate-limit that warrants retry.
    #[must_use]
    pub fn is_retryable(self) -> bool {
        self == Self::TooManyRequests
    }

    /// Whether the response indicates the requested resource is missing.
    ///
    /// - True for `NotFound` and `BadRequest`
    /// - The tracker returns `BadRequest` for some lookups against unknown ids
    #[must_use]
    pub fn is_missing(self) -> bool {
        matches!(self, Self::NotFound | Self::BadRequest)
    }
}

impl Display for ApiResponseKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadRequest => write!(f, "bad request"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::NotFound => write!(f, "not found"),
            Self::TooManyRequests => write!(f, "too many requests"),
            Self::Other => write!(f, "unexpected response"),
        }
    }
}
