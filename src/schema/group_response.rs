use serde::Deserialize;

use crate::{Group, Torrent};

/// Response for the `torrentgroup` action
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    /// Release information
    pub group: Group,
    /// Editions
    pub torrents: Vec<Torrent>,
}

#[cfg(feature = "mock")]
impl GroupResponse {
    /// Create a mock `GroupResponse` for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            group: Group::mock(),
            torrents: vec![Torrent::mock()],
        }
    }
}
