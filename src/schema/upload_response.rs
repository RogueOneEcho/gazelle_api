use serde::Deserialize;

/// Response for the `upload` action
#[derive(Clone, Debug, Deserialize)]
pub struct UploadResponse {
    /// Whether the torrent was modified to be private
    pub private: bool,
    /// Whether the source flag was added to the torrent
    pub source: bool,
    /// ID of the filled request, if the upload filled one
    #[serde(rename = "requestid")]
    pub request_id: Option<u32>,
    /// ID of the uploaded torrent
    ///
    /// Uses serde alias to handle both OPS (`torrentid`) and RED (`torrentId`) formats.
    #[serde(rename = "torrentId", alias = "torrentid")]
    pub torrent_id: u32,
    /// ID of the torrent group
    ///
    /// Uses serde alias to handle both OPS (`groupid`) and RED (`groupId`) formats.
    #[serde(rename = "groupId", alias = "groupid")]
    pub group_id: u32,
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
            torrent_id: 456,
            group_id: 123,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/upload_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/upload_response_red.json");

    #[test]
    fn deserialize_ops_format() {
        // OPS uses lowercase field names: torrentid, groupid
        let response: UploadResponse =
            serde_json::from_str(OPS_RESPONSE).expect("Failed to deserialize OPS format");
        assert_eq!(response.torrent_id, 123);
        assert_eq!(response.group_id, 456);
        assert!(response.private);
        assert!(response.source);
    }

    #[test]
    fn deserialize_red_format() {
        // RED uses camelCase field names: torrentId, groupId
        let response: UploadResponse =
            serde_json::from_str(RED_RESPONSE).expect("Failed to deserialize RED format");
        assert_eq!(response.torrent_id, 111);
        assert_eq!(response.group_id, 222);
        assert_eq!(response.request_id, Some(789));
        assert!(response.private);
    }
}
