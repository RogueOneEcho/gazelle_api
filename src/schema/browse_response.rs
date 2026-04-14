use crate::BrowseGroup;
use serde::Deserialize;

/// Response from the Gazelle browse action.
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrents-browse>
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResponse {
    /// Current page number (1-indexed).
    pub current_page: u32,
    /// Total number of pages.
    pub pages: u32,
    /// One entry per matching torrent group.
    pub results: Vec<BrowseGroup>,
}

#[cfg(feature = "mock")]
impl BrowseResponse {
    /// Create a mock [`BrowseResponse`] for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            current_page: 1,
            pages: 1,
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
    use crate::{BrowseResponse, Format, Media, Quality};

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/browse_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/browse_response_red.json");

    #[test]
    fn deserialize_ops() {
        let response: BrowseResponse =
            serde_json::from_str(OPS_RESPONSE).expect("fixture should deserialize");
        assert_eq!(response.current_page, 1);
        assert_eq!(response.pages, 20);
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
            serde_json::from_str(RED_RESPONSE).expect("fixture should deserialize");
        assert_eq!(response.current_page, 1);
        assert_eq!(response.pages, 50);
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

    #[test]
    fn browse_torrent_to_torrent_round_trip() {
        // Arrange
        let response: BrowseResponse =
            serde_json::from_str(RED_RESPONSE).expect("fixture should deserialize");
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
