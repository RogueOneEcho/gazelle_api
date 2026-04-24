use crate::prelude::*;

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
#[expect(
    clippy::indexing_slicing,
    reason = "test assertions on known fixture data"
)]
mod tests {
    use super::*;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_red.json");
    const MINIMAL_RESPONSE: &str = include_str!("../tests/fixtures/torrent_response_minimal.json");
    const OPS_EMPTY_RELEASE_TYPE: &str =
        include_str!("../tests/fixtures/torrent_response_ops_empty_release_type.json");
    const RED_EBOOK: &str = include_str!("../tests/fixtures/torrent_response_red_ebook.json");
    const RED_APP: &str = include_str!("../tests/fixtures/torrent_response_red_app.json");
    const RED_DSD: &str = include_str!("../tests/fixtures/torrent_response_red_dsd.json");
    const RED_DTS: &str = include_str!("../tests/fixtures/torrent_response_red_dts.json");
    const OPS_AAC: &str = include_str!("../tests/fixtures/torrent_response_ops_aac.json");
    const OPS_BD: &str = include_str!("../tests/fixtures/torrent_response_ops_bd.json");
    const RED_BLURAY: &str = include_str!("../tests/fixtures/torrent_response_red_bluray.json");
    const RED_V0: &str = include_str!("../tests/fixtures/torrent_response_red_v0.json");
    const OPS_Q8: &str = include_str!("../tests/fixtures/torrent_response_ops_q8.json");
    const RED_CASSETTE: &str = include_str!("../tests/fixtures/torrent_response_red_cassette.json");
    const RED_ID_CONFLICT: &str =
        include_str!("../tests/fixtures/torrent_response_red_id_conflict.json");
    const OPS_ID_CONFLICT: &str =
        include_str!("../tests/fixtures/torrent_response_ops_id_conflict.json");

    #[test]
    fn deserialize_ops_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(OPS_RESPONSE).expect("should deserialize");

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
        assert_eq!(response.torrent.media, Media::WEB);
        assert_eq!(response.torrent.format, Format::FLAC);
        assert_eq!(response.torrent.encoding, Quality::Lossless);
        assert_eq!(response.group.release_type, ReleaseTypeId(9));
    }

    #[test]
    fn deserialize_red_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_RESPONSE).expect("should deserialize");

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
        assert_eq!(response.torrent.media, Media::WEB);
    }

    #[test]
    fn deserialize_minimal_torrent_response() {
        // Arrange & Act
        let response: TorrentResponse =
            json_from_str(MINIMAL_RESPONSE).expect("should deserialize");

        // Assert - Minimal response parses correctly
        assert_eq!(response.group.id, 3);
        assert_eq!(response.group.name, "Minimal Album");
        assert!(response.group.tags.is_empty());
        assert!(response.group.music_info.is_none());
        assert_eq!(response.torrent.id, 3000);
        assert_eq!(response.torrent.format, Format::MP3);
        assert_eq!(response.torrent.remastered, Some(false));
        assert!(response.torrent.scene);
    }

    #[test]
    fn deserialize_torrent_music_info() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_RESPONSE).expect("should deserialize");

        // Assert - Music info is properly populated
        let music_info = response.group.music_info.expect("music_info should exist");
        assert_eq!(music_info.artists.len(), 1);
        assert_eq!(music_info.artists[0].name, "Test Artist");
    }

    #[test]
    fn deserialize_torrent_numeric_fields() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_RESPONSE).expect("should deserialize");

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
        let response: TorrentResponse = json_from_str(OPS_RESPONSE).expect("should deserialize");

        // Assert - File list is preserved
        assert!(response.torrent.file_list.contains("Track.flac"));

        // Assert - get_flacs works with fixture data
        let flacs = response.torrent.get_flacs();
        assert_eq!(flacs.len(), 1);
    }

    /// OPS returns `releaseType: ""` for some torrents where the field is
    /// unset in the database.
    ///
    /// <https://github.com/RogueOneEcho/gazelle_api/issues/5>
    #[test]
    fn deserialize_ops_empty_release_type() {
        // Arrange & Act
        let response: TorrentResponse =
            json_from_str(OPS_EMPTY_RELEASE_TYPE).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99001);
        assert_eq!(response.group.name, "Mock Author - Mock Audiobook Title");
        assert_eq!(response.group.category_id, Category::Audiobooks);
        assert_eq!(response.group.category_name, "Audiobooks");
        assert_eq!(response.torrent.id, 99002);
        assert_eq!(response.torrent.format, Format::AAC);
        assert_eq!(response.group.release_type, ReleaseTypeId(0));
    }

    /// RED returns `releaseType: 0` for non-Music categories (unlike OPS which returns `""`).
    /// E-Books have empty media, format, and encoding fields.
    #[test]
    fn deserialize_red_ebook() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_EBOOK).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99101);
        assert_eq!(response.group.category_id, Category::EBooks);
        assert_eq!(response.group.category_name, "E-Books");
        assert_eq!(response.group.release_type, ReleaseTypeId(0));
        assert_eq!(response.torrent.media, Media::Other(String::new()));
        assert_eq!(response.torrent.format, Format::Other(String::new()));
        assert_eq!(response.torrent.encoding, Quality::Other(String::new()));
    }

    /// Applications category with neutralleech and empty media/format/encoding.
    #[test]
    fn deserialize_red_app() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_APP).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99201);
        assert_eq!(response.group.category_id, Category::Applications);
        assert_eq!(response.group.category_name, "Applications");
        assert_eq!(response.group.release_type, ReleaseTypeId(0));
        assert_eq!(response.torrent.is_neutralleech, Some(true));
        assert_eq!(response.torrent.media, Media::Other(String::new()));
        assert_eq!(response.torrent.format, Format::Other(String::new()));
        assert_eq!(response.torrent.encoding, Quality::Other(String::new()));
    }

    /// DSD format and quality on SACD media, all RED-only enum variants.
    #[test]
    fn deserialize_red_dsd() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_DSD).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99301);
        assert_eq!(response.group.category_id, Category::Music);
        assert_eq!(response.group.release_type, ReleaseTypeId(1));
        assert_eq!(response.torrent.media, Media::SACD);
        assert_eq!(response.torrent.format, Format::DSD);
        assert_eq!(response.torrent.encoding, Quality::DSD64);
    }

    /// DTS format on DVD media with a freeform bitrate value.
    #[test]
    fn deserialize_red_dts() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_DTS).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99401);
        assert_eq!(response.torrent.media, Media::DVD);
        assert_eq!(response.torrent.format, Format::DTS);
        assert_eq!(response.torrent.encoding, Quality::Other("1510".to_owned()));
    }

    /// OPS AAC format with 256 kbps quality.
    #[test]
    fn deserialize_ops_aac() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(OPS_AAC).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99501);
        assert_eq!(response.group.release_type, ReleaseTypeId(9));
        assert_eq!(response.torrent.media, Media::WEB);
        assert_eq!(response.torrent.format, Format::AAC);
        assert_eq!(response.torrent.encoding, Quality::_256);
    }

    /// OPS BD media with AC3 format and freeform quality value.
    #[test]
    fn deserialize_ops_bd() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(OPS_BD).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99601);
        assert_eq!(response.group.release_type, ReleaseTypeId(11));
        assert_eq!(response.torrent.media, Media::BD);
        assert_eq!(response.torrent.format, Format::AC3);
        assert_eq!(
            response.torrent.encoding,
            Quality::Other("384 kbps".to_owned())
        );
    }

    /// RED Blu-Ray media (distinct from OPS BD) with 24bit Lossless quality.
    #[test]
    fn deserialize_red_bluray() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_BLURAY).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99701);
        assert_eq!(response.torrent.media, Media::BluRay);
        assert_eq!(response.torrent.format, Format::FLAC);
        assert_eq!(response.torrent.encoding, Quality::Lossless24);
    }

    /// MP3 V0 (VBR) on CD media.
    #[test]
    fn deserialize_red_v0() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_V0).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99801);
        assert_eq!(response.torrent.media, Media::CD);
        assert_eq!(response.torrent.format, Format::MP3);
        assert_eq!(response.torrent.encoding, Quality::V0);
    }

    /// OPS q8.x (VBR) quality, an OPS-only variant.
    #[test]
    fn deserialize_ops_q8() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(OPS_Q8).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 99901);
        assert_eq!(response.group.release_type, ReleaseTypeId(6));
        assert_eq!(response.torrent.media, Media::CD);
        assert_eq!(response.torrent.format, Format::MP3);
        assert_eq!(response.torrent.encoding, Quality::Q8x);
    }

    /// Cassette media with AAC format.
    #[test]
    fn deserialize_red_cassette() {
        // Arrange & Act
        let response: TorrentResponse = json_from_str(RED_CASSETTE).expect("should deserialize");

        // Assert
        assert_eq!(response.group.id, 100_001);
        assert_eq!(response.torrent.media, Media::Cassette);
        assert_eq!(response.torrent.format, Format::AAC);
        assert_eq!(response.torrent.encoding, Quality::_256);
    }

    /// Verify that `Display` output matches the original JSON string for all fixtures.
    ///
    /// [`UploadForm::to_form`] uses `Display` to produce the string values sent to the
    /// upload endpoint, so the output must exactly match the API's canonical strings.
    #[test]
    fn display_round_trip() {
        let fixtures = [
            OPS_RESPONSE,
            RED_RESPONSE,
            MINIMAL_RESPONSE,
            OPS_EMPTY_RELEASE_TYPE,
            RED_EBOOK,
            RED_APP,
            RED_DSD,
            RED_DTS,
            OPS_AAC,
            OPS_BD,
            RED_BLURAY,
            RED_V0,
            OPS_Q8,
            RED_CASSETTE,
        ];
        for fixture in fixtures {
            let raw: JsonValue = json_from_str(fixture).expect("should deserialize as value");
            let response: TorrentResponse =
                json_from_str(fixture).expect("should deserialize as response");
            let torrent = &raw["torrent"];
            assert_eq!(
                response.torrent.media.to_string(),
                torrent["media"].as_str().expect("media should be a string"),
                "media mismatch in fixture with torrent id {}",
                torrent["id"]
            );
            assert_eq!(
                response.torrent.format.to_string(),
                torrent["format"]
                    .as_str()
                    .expect("format should be a string"),
                "format mismatch in fixture with torrent id {}",
                torrent["id"]
            );
            assert_eq!(
                response.torrent.encoding.to_string(),
                torrent["encoding"]
                    .as_str()
                    .expect("encoding should be a string"),
                "encoding mismatch in fixture with torrent id {}",
                torrent["id"]
            );
        }
    }

    /// RED uses ID 17 for Demo. The raw `ReleaseTypeId` deserializes as 17,
    /// and `from_int_red` resolves it to `Demo`.
    #[test]
    fn deserialize_red_id_conflict() {
        let response: TorrentResponse = json_from_str(RED_ID_CONFLICT).expect("should deserialize");
        assert_eq!(response.group.release_type, ReleaseTypeId(17));
        assert_eq!(
            ReleaseType::from_int_red(response.group.release_type),
            Some(ReleaseType::Demo)
        );
    }

    /// OPS uses ID 17 for DJ Mix (not Demo). The raw `ReleaseTypeId` deserializes
    /// as 17, but `from_int_ops` correctly resolves it to `DjMix`.
    #[test]
    fn deserialize_ops_id_conflict() {
        let response: TorrentResponse = json_from_str(OPS_ID_CONFLICT).expect("should deserialize");
        assert_eq!(response.group.release_type, ReleaseTypeId(17));
        assert_eq!(
            ReleaseType::from_int_ops(response.group.release_type),
            Some(ReleaseType::DjMix)
        );
    }
}
