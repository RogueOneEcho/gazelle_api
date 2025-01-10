use serde::Deserialize;

use crate::{Group, Torrent};

/// Response for the `torrentgroup` action
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    /// Release information
    pub group: Group,
    /// Editions
    pub torrents: Vec<Torrent>,
}
