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

#[cfg(feature = "mock")]
impl TorrentResponse {
    /// Create a mock `TorrentResponse` for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            group: Group::mock(),
            torrent: Torrent::mock(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_red.json");
    const MINIMAL_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_minimal.json");

    #[test]
    fn deserialize_ops_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - OPS has trumpable
        assert_eq!(response.torrent.trumpable, Some(false));
        // Assert - OPS lacks RED-specific fields
        assert!(response.torrent.lossy_web_approved.is_none());
        assert!(response.torrent.lossy_master_approved.is_none());
        assert!(response.torrent.is_neutralleech.is_none());
        assert!(response.torrent.is_freeload.is_none());
        // Assert - OPS uses wikiBBcode not bbBody, so bb_body is None
        assert!(response.group.bb_body.is_none());

        // Assert - Core fields
        assert_eq!(response.group.id, 16352);
        assert_eq!(response.group.name, "Test Album");
        assert_eq!(response.torrent.id, 1_520_678);
        assert_eq!(response.torrent.media, "WEB");
        assert_eq!(response.torrent.format, "FLAC");
        assert_eq!(response.torrent.encoding, "Lossless");
    }

    #[test]
    fn deserialize_red_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - RED uses bbBody
        assert!(response.group.bb_body.is_some());

        // Assert - RED-specific fields are present
        assert_eq!(response.torrent.trumpable, Some(false));
        assert_eq!(response.torrent.lossy_web_approved, Some(false));
        assert_eq!(response.torrent.lossy_master_approved, Some(false));
        assert_eq!(response.torrent.is_neutralleech, Some(false));
        assert_eq!(response.torrent.is_freeload, Some(false));

        // Assert - RED has has_snatched field
        assert_eq!(response.torrent.has_snatched, Some(false));

        // Assert - Core fields
        assert_eq!(response.group.id, 8126);
        assert_eq!(response.group.name, "Test Album");
        assert_eq!(response.torrent.id, 12483);
        assert_eq!(response.torrent.media, "WEB");
    }

    #[test]
    fn deserialize_minimal_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(MINIMAL_RESPONSE).unwrap();

        // Assert - Minimal response parses correctly
        assert_eq!(response.group.id, 3);
        assert_eq!(response.group.name, "Minimal Album");
        assert!(response.group.tags.is_empty());
        assert!(response.group.music_info.is_none());
        assert_eq!(response.torrent.id, 3000);
        assert_eq!(response.torrent.format, "MP3");
        assert!(!response.torrent.remastered);
        assert!(response.torrent.scene);
    }

    #[test]
    fn deserialize_torrent_music_info() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - Music info is properly populated
        let music_info = response.group.music_info.expect("music_info should exist");
        assert_eq!(music_info.artists.len(), 1);
        assert_eq!(music_info.artists[0].name, "Test Artist");
    }

    #[test]
    fn deserialize_torrent_numeric_fields() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - Numeric fields parse correctly
        assert_eq!(response.torrent.file_count, 2);
        assert_eq!(response.torrent.size, 30_487_522);
        assert_eq!(response.torrent.seeders, 15);
        assert_eq!(response.torrent.leechers, 0);
        assert_eq!(response.torrent.snatched, 55);
        assert_eq!(response.group.year, 2015);
        assert_eq!(response.torrent.remaster_year, Some(2015));
    }

    #[test]
    fn deserialize_torrent_file_list() {
        // Arrange & Act
        let response: TorrentResponse = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - File list is preserved
        assert!(response.torrent.file_list.contains("Track.flac"));

        // Assert - get_flacs works with fixture data
        let flacs = response.torrent.get_flacs();
        assert_eq!(flacs.len(), 1);
    }
}
