use serde::Deserialize;

/// Response for the `upload` action
#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize)]
pub struct UploadResponse {
    /// If `true` you will need to download the torrent file.
    pub private: bool,
    /// If `true` you will need to download the torrent file.
    pub source: bool,
    #[serde(rename = "requestid")]
    pub request_id: Option<u32>,
    torrentid: Option<u32>,
    groupid: Option<u32>,
    torrentId: Option<u32>,
    groupId: Option<u32>,
}

impl UploadResponse {
    #[must_use]
    pub fn get_torrent_id(&self) -> u32 {
        self.torrentid
            .unwrap_or_else(|| self.torrentId.unwrap_or_default())
    }
    #[must_use]
    pub fn get_group_id(&self) -> u32 {
        self.groupid
            .unwrap_or_else(|| self.groupId.unwrap_or_default())
    }
}

#[cfg(feature = "mock")]
impl UploadResponse {
    /// Create a mock `UploadResponse` for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            private: true,
            source: true,
            request_id: None,
            torrentid: Some(456),
            groupid: Some(123),
            torrentId: None,
            groupId: None,
        }
    }
}
