use serde::Deserialize;

use crate::{Group, Torrent};

/// Response for the `torrentgroup` action
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    /// Release information
    pub group: Group,
    /// Editions
    pub torrents: Vec<Torrent>,
}

#[cfg(feature = "mock")]
impl GroupResponse {
    /// Create a mock `GroupResponse` for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            group: Group::mock(),
            torrents: vec![Torrent::mock()],
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/group_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/group_response_red.json");

    #[test]
    fn deserialize_ops_group_response() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - OPS uses wikiBBcode not bbBody, so bb_body is None
        assert!(response.group.bb_body.is_none());

        // Assert - Core group fields
        assert_eq!(response.group.id, 1_161_539);
        assert_eq!(response.group.name, "Test Album");
        assert_eq!(response.group.year, 2024);
        assert!(!response.group.vanity_house);

        // Assert - OPS has trumpable on torrents
        assert_eq!(response.torrents[0].trumpable, Some(false));
        // Assert - OPS lacks RED-specific torrent fields
        assert!(response.torrents[0].lossy_web_approved.is_none());
        assert!(response.torrents[0].lossy_master_approved.is_none());
        assert!(response.torrents[0].is_neutralleech.is_none());
        assert!(response.torrents[0].is_freeload.is_none());
    }

    #[test]
    fn deserialize_red_group_response() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - RED uses bbBody, so bb_body has value
        assert!(response.group.bb_body.is_some());

        // Assert - Core group fields
        assert_eq!(response.group.id, 629);
        assert_eq!(response.group.name, "Test Album");
        assert_eq!(response.group.year, 1982);
        assert!(!response.group.vanity_house);

        // Assert - RED has trumpable and other RED-specific fields
        assert_eq!(response.torrents[0].trumpable, Some(false));
        assert_eq!(response.torrents[0].lossy_web_approved, Some(false));
        assert_eq!(response.torrents[2].trumpable, Some(true));
        assert_eq!(response.torrents[2].lossy_web_approved, Some(true));
        assert_eq!(response.torrents[2].is_neutralleech, Some(true));
    }

    #[test]
    fn deserialize_group_with_multiple_torrents() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - Multiple editions
        assert_eq!(response.torrents.len(), 2);

        // Assert - First torrent (24bit)
        let first_torrent = &response.torrents[0];
        assert_eq!(first_torrent.id, 2_492_578);
        assert_eq!(first_torrent.media, "WEB");
        assert_eq!(first_torrent.encoding, "24bit Lossless");

        // Assert - Second torrent (16bit)
        let second_torrent = &response.torrents[1];
        assert_eq!(second_torrent.id, 1_520_678);
        assert_eq!(second_torrent.encoding, "Lossless");
    }

    #[test]
    fn deserialize_ops_group_music_info() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - OPS music info with nested artist structure (extra fields ignored)
        let music_info = response.group.music_info.expect("music_info should exist");
        assert_eq!(music_info.artists.len(), 1);
        assert_eq!(music_info.artists[0].name, "Test Artist");
        // OPS has arranger field
        assert!(music_info.arranger.is_some());
        assert!(music_info.arranger.as_ref().unwrap().is_empty());
    }

    #[test]
    fn deserialize_red_group_music_info() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - RED music info (simpler artist structure)
        let music_info = response.group.music_info.expect("music_info should exist");
        assert_eq!(music_info.artists.len(), 1);
        assert_eq!(music_info.artists[0].name, "Test Artist");
        // RED lacks arranger field
        assert!(music_info.arranger.is_none());
    }

    #[test]
    fn deserialize_group_tags() {
        // Arrange & Act
        let ops_response: GroupResponse = serde_json::from_str(OPS_RESPONSE).unwrap();
        let red_response: GroupResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - OPS tags
        assert_eq!(ops_response.group.tags.len(), 1);
        assert!(ops_response.group.tags.contains(&"electronic".to_owned()));

        // Assert - RED tags
        assert_eq!(red_response.group.tags.len(), 3);
        assert!(red_response.group.tags.contains(&"rock".to_owned()));
        assert!(red_response.group.tags.contains(&"punk".to_owned()));
    }

    #[test]
    fn deserialize_group_torrent_uploaders() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - Different uploaders for each torrent
        assert_eq!(response.torrents[0].user_id, 2001);
        assert_eq!(response.torrents[0].username, "user_a");
        assert_eq!(response.torrents[1].user_id, 2002);
        assert_eq!(response.torrents[1].username, "user_b");
    }

    #[test]
    fn deserialize_red_group_with_three_editions() {
        // Arrange & Act
        let response: GroupResponse = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - Three different editions
        assert_eq!(response.torrents.len(), 3);
        assert_eq!(response.torrents[0].media, "Vinyl");
        assert_eq!(response.torrents[1].media, "CD");
        assert_eq!(response.torrents[2].media, "WEB");
    }
}
