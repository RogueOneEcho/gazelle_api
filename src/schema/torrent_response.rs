use serde::Deserialize;

use crate::{Group, Torrent};

/// Response for the `torrent` action
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentResponse {
    /// Release information
    pub group: Group,
    /// Edition information
    pub torrent: Torrent,
}
