use crate::prelude::*;

/// A release.
///
/// Typically represents an album, EP, or single which may contain multiple editions.
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent-group>
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    /// Album info formatted as HTML
    pub wiki_body: String,
    /// Album info formatted as BB code
    pub bb_body: Option<String>,
    /// Cover image URL
    pub wiki_image: String,
    /// ID number
    pub id: u32,
    /// Release Name
    pub name: String,
    /// Release Year
    pub year: u16,
    /// Record label
    ///
    /// *RED only*
    pub record_label: Option<String>,
    /// Release catalogue number
    ///
    /// *RED only*
    pub catalogue_number: Option<String>,
    /// Release type
    pub release_type: ReleaseTypeId,
    /// Category
    pub category_id: Category,
    /// Category name
    pub category_name: String,
    /// Time of last logged event
    pub time: String,
    /// Is this a Vanity House release?
    pub vanity_house: bool,
    /// Is this release bookmarked?
    pub is_bookmarked: bool,
    /// Tags
    pub tags: Vec<String>,
    /// Release credits.
    ///
    /// Artists, composers, etc.
    pub music_info: Option<Credits>,
}

#[cfg(feature = "mock")]
impl Group {
    /// Create a mock Group for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            id: 123,
            name: "Test Album".to_owned(),
            year: 2020,
            record_label: Some("Test Label".to_owned()),
            catalogue_number: Some("TEST-001".to_owned()),
            category_id: Category::Music,
            category_name: "Music".to_owned(),
            time: "2020-01-01 00:00:00".to_owned(),
            vanity_house: false,
            is_bookmarked: false,
            music_info: None,
            tags: vec!["rock".to_owned()],
            wiki_body: "Test wiki body".to_owned(),
            bb_body: None,
            wiki_image: "https://example.com/image.jpg".to_owned(),
            release_type: ReleaseTypeId(1),
        }
    }
}
