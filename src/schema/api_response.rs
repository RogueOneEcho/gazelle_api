use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Status
    /// Either `success` or `failure`
    pub status: String,
    /// API response
    /// Only popualted on success
    pub response: Option<T>,
    /// Explanation of error
    pub error: Option<String>,
}
