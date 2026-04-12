use serde::{Deserialize, Serialize};

/// Sort direction for [`BrowseRequest`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderWay {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

impl OrderWay {
    /// Query parameter value for the Gazelle `order_way` parameter.
    #[must_use]
    pub fn as_query(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}
