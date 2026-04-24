use crate::prelude::*;

/// Response from the Gazelle browse action.
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrents-browse>
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResponse {
    /// Current page number (1-indexed).
    ///
    /// - `None` when the browse returned no results
    pub current_page: Option<u32>,
    /// Total number of pages.
    ///
    /// - `None` when the browse returned no results
    pub pages: Option<u32>,
    /// One entry per matching torrent group.
    pub results: Vec<BrowseGroup>,
}

#[cfg(feature = "mock")]
impl BrowseResponse {
    /// Create a mock [`BrowseResponse`] for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            current_page: Some(1),
            pages: Some(1),
            results: vec![BrowseGroup::mock()],
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::indexing_slicing,
    reason = "test assertions on known fixture data"
)]
mod tests {
    use super::*;
    use crate::client::deserialize;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/browse_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/browse_response_red.json");
    const OPS_CASING: &str = include_str!("../tests/fixtures/browse_response_ops_casing.json");
    const RED_CASING: &str = include_str!("../tests/fixtures/browse_response_red_casing.json");
    const OPS_EMPTY: &str = include_str!("../tests/fixtures/browse_response_ops_empty.json");
    const RED_EMPTY: &str = include_str!("../tests/fixtures/browse_response_red_empty.json");

    #[test]
    fn deserialize_ops() {
        let response: BrowseResponse =
            json_from_str(OPS_RESPONSE).expect("fixture should deserialize");
        assert_eq!(response.current_page, Some(1));
        assert_eq!(response.pages, Some(20));
        assert_eq!(response.results.len(), 1);
        let group = &response.results[0];
        assert_eq!(group.group_id, 100_200);
        assert_eq!(group.group_name, "Mock Album");
        assert_eq!(group.torrents.len(), 2);
        assert_eq!(group.torrents[0].torrent_id, 3_000_001);
        assert_eq!(group.torrents[0].remastered, None);
        assert_eq!(group.torrents[0].media, Media::CD);
        assert_eq!(group.torrents[0].encoding, Quality::Lossless);
        assert_eq!(group.torrents[1].torrent_id, 3_000_002);
        assert_eq!(group.torrents[1].encoding, Quality::Lossless24);
    }

    #[test]
    fn deserialize_red() {
        let response: BrowseResponse =
            json_from_str(RED_RESPONSE).expect("fixture should deserialize");
        assert_eq!(response.current_page, Some(1));
        assert_eq!(response.pages, Some(50));
        assert_eq!(response.results.len(), 1);
        let group = &response.results[0];
        assert_eq!(group.group_id, 200_300);
        assert_eq!(group.group_name, "Mock Album");
        assert_eq!(group.torrents.len(), 2);
        assert_eq!(group.torrents[0].torrent_id, 6_000_001);
        assert_eq!(group.torrents[0].remastered, Some(true));
        assert_eq!(group.torrents[0].media, Media::WEB);
        assert_eq!(group.torrents[0].encoding, Quality::Lossless);
        assert_eq!(group.torrents[1].torrent_id, 6_000_002);
        assert_eq!(group.torrents[1].remastered, Some(false));
    }

    /// OPS returns `"Concert recording"` (lowercase r) from the DB `release_type` table.
    #[test]
    fn deserialize_ops_casing() {
        let response: BrowseResponse =
            json_from_str(OPS_CASING).expect("fixture should deserialize");
        let group = &response.results[0];
        assert_eq!(group.release_type, ReleaseType::ConcertRecording);
    }

    /// RED returns `"Concert Recording"` (uppercase R) from `upload.js`.
    #[test]
    fn deserialize_red_casing() {
        let response: BrowseResponse =
            json_from_str(RED_CASING).expect("fixture should deserialize");
        let group = &response.results[0];
        assert_eq!(group.release_type, ReleaseType::ConcertRecording);
    }

    /// OPS omits `currentPage` and `pages` when a browse returns zero results.
    #[test]
    fn deserialize_ops_empty() {
        let api_response = deserialize::<BrowseResponse>(OPS_EMPTY.to_owned())
            .expect("empty ops response should deserialize");
        let response = api_response
            .response
            .expect("success response should contain body");
        assert_eq!(response.current_page, None);
        assert_eq!(response.pages, None);
        assert!(response.results.is_empty());
    }

    /// RED omits `currentPage` and `pages` when a browse returns zero results.
    #[test]
    fn deserialize_red_empty() {
        let api_response = deserialize::<BrowseResponse>(RED_EMPTY.to_owned())
            .expect("empty red response should deserialize");
        let response = api_response
            .response
            .expect("success response should contain body");
        assert_eq!(response.current_page, None);
        assert_eq!(response.pages, None);
        assert!(response.results.is_empty());
    }

    #[test]
    fn browse_torrent_to_torrent_round_trip() {
        // Arrange
        let response: BrowseResponse =
            json_from_str(RED_RESPONSE).expect("fixture should deserialize");
        let browse_torrent = &response.results[0].torrents[0];

        // Act
        let torrent = browse_torrent.to_torrent();

        // Assert
        assert_eq!(torrent.id, 6_000_001);
        assert_eq!(torrent.media, Media::WEB);
        assert_eq!(torrent.format, Format::FLAC);
        assert_eq!(torrent.encoding, Quality::Lossless);
        assert_eq!(torrent.remastered, Some(true));
        assert_eq!(torrent.remaster_catalogue_number, "MOCK-100");
    }
}
