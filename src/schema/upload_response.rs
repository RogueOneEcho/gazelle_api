use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub private: bool,
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
