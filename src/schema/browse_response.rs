use crate::{Credit, Format, Media, Quality, ReleaseType, Torrent};
use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};
use std::fmt::{Formatter, Result as FmtResult};

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

/// A single group entry in a [`BrowseResponse`].
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseGroup {
    /// Group ID.
    ///
    /// - OPS: string
    /// - RED: number
    #[serde(deserialize_with = "string_or_u32")]
    pub group_id: u32,
    /// Group (album) name.
    pub group_name: String,
    /// Primary artist name.
    pub artist: String,
    /// Cover art URL.
    pub cover: String,
    /// Tag names for the group.
    pub tags: Vec<String>,
    /// Whether the group is bookmarked by the authenticated user.
    pub bookmarked: bool,
    /// Vanity house flag.
    pub vanity_house: bool,
    /// Release year of the group.
    pub group_year: u32,
    /// Release type.
    #[serde(deserialize_with = "release_type_from_display")]
    pub release_type: ReleaseType,
    /// Group upload time.
    ///
    /// - OPS: `YYYY-MM-DD HH:MM:SS`
    /// - RED: unix timestamp
    pub group_time: String,
    /// Size of the largest torrent in the group (bytes).
    pub max_size: u64,
    /// Total number of snatches across all torrents.
    pub total_snatched: u32,
    /// Total number of seeders across all torrents.
    pub total_seeders: u32,
    /// Total number of leechers across all torrents.
    pub total_leechers: u32,
    /// All torrents in the group, not only those matching the browse filter.
    pub torrents: Vec<BrowseTorrent>,
}

/// A single torrent entry in a [`BrowseGroup`].
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "mirrors Gazelle API JSON shape"
)]
pub struct BrowseTorrent {
    /// Torrent ID.
    pub torrent_id: u32,
    /// Edition ID.
    pub edition_id: u32,
    /// Credited artists on this torrent.
    pub artists: Vec<Credit>,
    /// Media type.
    pub media: Media,
    /// Format.
    pub format: Format,
    /// Encoding (maps to Gazelle `encoding` field).
    pub encoding: Quality,
    /// Remaster flag.
    ///
    /// *RED only*
    pub remastered: Option<bool>,
    /// Remaster year if applicable.
    pub remaster_year: Option<u16>,
    /// Edition record label.
    ///
    /// *OPS only*
    pub remaster_record_label: Option<String>,
    /// Edition catalogue number.
    pub remaster_catalogue_number: String,
    /// Edition title.
    pub remaster_title: String,
    /// Whether the torrent has a log file.
    pub has_log: bool,
    /// Log score (0-100).
    pub log_score: i32,
    /// Whether the torrent has a cue file.
    pub has_cue: bool,
    /// Scene release flag.
    pub scene: bool,
    /// Vanity house flag.
    pub vanity_house: bool,
    /// Number of files in the torrent.
    pub file_count: u32,
    /// Upload datetime in `YYYY-MM-DD HH:MM:SS` format.
    pub time: String,
    /// Total size in bytes.
    pub size: u64,
    /// Number of snatches.
    pub snatches: u32,
    /// Number of seeders.
    pub seeders: u32,
    /// Number of leechers.
    pub leechers: u32,
    /// Whether the torrent is freeleech.
    pub is_freeleech: bool,
    /// Whether the torrent is neutral leech.
    pub is_neutral_leech: bool,
    /// Whether the torrent is personal freeleech for the authenticated user.
    pub is_personal_freeleech: bool,
    /// Whether a freeleech token can be used.
    pub can_use_token: bool,
    /// Whether the authenticated user has snatched this torrent.
    pub has_snatched: bool,
    /// Leech status.
    ///
    /// *RED only*
    pub leech_status: Option<u32>,
    /// Whether the torrent is a freeload.
    ///
    /// *RED only*
    pub is_freeload: Option<bool>,
    /// Whether the torrent is trumpable.
    ///
    /// *RED only*
    pub trumpable: Option<bool>,
}

impl BrowseTorrent {
    /// Construct a [`Torrent`] from browse data.
    ///
    /// - Fields not available in the browse response are left at their defaults
    /// - `remaster_record_label` is empty on RED because the browse endpoint does not include it
    #[must_use]
    pub fn to_torrent(&self) -> Torrent {
        Torrent {
            id: self.torrent_id,
            media: self.media.clone(),
            format: self.format.clone(),
            encoding: self.encoding.clone(),
            remastered: self.remastered,
            remaster_year: self.remaster_year,
            remaster_title: self.remaster_title.clone(),
            remaster_record_label: self.remaster_record_label.clone().unwrap_or_default(),
            remaster_catalogue_number: self.remaster_catalogue_number.clone(),
            scene: self.scene,
            has_log: self.has_log,
            has_cue: self.has_cue,
            log_score: self.log_score,
            file_count: self.file_count,
            size: self.size,
            seeders: self.seeders,
            leechers: self.leechers,
            snatched: self.snatches,
            has_snatched: Some(self.has_snatched),
            trumpable: self.trumpable,
            is_freeload: self.is_freeload,
            time: self.time.clone(),
            ..Torrent::default()
        }
    }
}

/// Deserialize a [`ReleaseType`] from its display name string.
fn release_type_from_display<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ReleaseType, D::Error> {
    let s = String::deserialize(deserializer)?;
    ReleaseType::from_display(&s)
        .ok_or_else(|| de::Error::custom(format!("unrecognized release type: {s}")))
}

/// Deserialize a `u32` from either a number or a string.
///
/// - OPS browse responses return some IDs as strings
fn string_or_u32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    struct StringOrU32Visitor;

    impl Visitor<'_> for StringOrU32Visitor {
        type Value = u32;

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("a u32 or a string containing a u32")
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            u32::try_from(value).map_err(de::Error::custom)
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(StringOrU32Visitor)
}

#[cfg(test)]
#[expect(
    clippy::indexing_slicing,
    reason = "test assertions on known fixture data"
)]
mod tests {
    use super::*;

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
