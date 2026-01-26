use serde::{Deserialize, Serialize};

/// Wrapper for Gazelle API responses
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Status.
    ///
    /// Either `success` or `failure`.
    pub status: String,
    /// API response payload.
    ///
    /// Only populated on success.
    pub response: Option<T>,
    /// Explanation of error
    pub error: Option<String>,
}
