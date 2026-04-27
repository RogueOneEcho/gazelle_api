use crate::prelude::*;

/// The operation that failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ThisError, Serialize, Deserialize)]
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
