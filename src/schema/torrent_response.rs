use serde::Deserialize;

use crate::{Group, Torrent};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentResponse {
    pub group: Group,
    pub torrent: Torrent,
}
