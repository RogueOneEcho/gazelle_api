use serde::Deserialize;

use crate::{Group, Torrent};

/// Response for the `torrent` action
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentResponse {
    /// Release information
    pub group: Group,
    /// Edition information
    pub torrent: Torrent,
}
