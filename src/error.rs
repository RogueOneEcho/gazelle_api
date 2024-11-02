use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub action: Action,
    pub message: Option<String>,
    pub status_code: Option<u16>,
    #[serde(skip)]
    pub inner: Option<InnerError>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Action {
    GetTorrent,
    GetTorrentGroup,
    GetTorrentFile,
    UploadTorrent,
}

#[derive(Debug)]
pub enum InnerError {
    #[allow(clippy::absolute_paths)]
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}
