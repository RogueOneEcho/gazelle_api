use crate::prelude::*;

/// An error from the API response.
#[derive(Clone, Debug, ThisError)]
#[error("{message}")]
pub struct ApiResponseError {
    pub message: String,
    pub status: u16,
}
