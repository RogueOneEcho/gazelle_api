use crate::prelude::*;

/// The source of a [`GazelleError`].
#[derive(Debug)]
pub enum ErrorSource {
    Reqwest(ReqwestError),
    SerdeJson(JsonError),
    Io(IoError),
    ApiResponse(ApiResponseError),
    Stringified(String),
}

impl Display for ErrorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
