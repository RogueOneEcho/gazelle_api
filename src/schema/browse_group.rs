use crate::prelude::*;

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

#[cfg(feature = "mock")]
impl BrowseGroup {
    /// Create a mock [`BrowseGroup`] for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            group_id: 123,
            group_name: "Test Album".to_owned(),
            artist: "Test Artist".to_owned(),
            cover: "https://example.com/cover.jpg".to_owned(),
            tags: vec!["rock".to_owned()],
            bookmarked: false,
            vanity_house: false,
            group_year: 2020,
            release_type: ReleaseType::Album,
            group_time: "2020-01-01 00:00:00".to_owned(),
            max_size: 500_000_000,
            total_snatched: 100,
            total_seeders: 50,
            total_leechers: 2,
            torrents: vec![BrowseTorrent::mock()],
        }
    }
}

/// Deserialize a [`ReleaseType`] from its display name string.
fn release_type_from_display<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ReleaseType, D::Error> {
    let s = String::deserialize(deserializer)?;
    ReleaseType::from_display(&s)
        .ok_or_else(|| DeError::custom(format!("unrecognized release type: {s}")))
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

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            u32::try_from(value).map_err(DeError::custom)
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map_err(DeError::custom)
        }
    }

    deserializer.deserialize_any(StringOrU32Visitor)
}
