use serde::Deserialize;

use crate::Credits;

/// A release
/// Typically representing an album, EP, or single which may contain multiple editions.
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
    pub record_label: String,
    /// Release catalogue number
    pub catalogue_number: String,
    /// Index of release type
    ///
    /// 1: `Album`
    /// 3: `Soundtrack`
    /// 5: `EP`
    /// 6: `Anthology`
    /// 7: `Compilation`
    /// 9: `Single`
    /// 11: `Live album`
    /// 13: `Remix`
    /// 14: `Bootleg`
    /// 15: `Interview`
    /// 16: `Mixtape`
    /// 17: `Demo` (RED only)
    /// 18: `Concert Recording` (RED only)
    /// 19: `DJ Mix` (RED only)
    /// 21: `Unknown`
    /// <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L582-L595>
    pub release_type: u8,
    /// ID number of the category
    ///
    /// *CAUTION: This index is inexplicably different to the [`UploadForm`] `category_id`*
    ///
    /// 0: `Music`
    /// 1: `Applications`
    /// 2: `E-Books`
    /// 3: `Audiobooks`
    /// 4: `E-Learning Videos`
    /// 5: `Comedy`
    /// 6: `Comics`
    /// <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L702-L710>
    pub category_id: u8,
    /// 0: `Music`
    /// 1: `Applications`
    /// 2: `E-Books`
    /// 3: `Audiobooks`
    /// 4: `E-Learning Videos`
    /// 5: `Comedy`
    /// 6: `Comics`
    /// <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L702-L710>
    pub category_name: String,
    /// Time of last logged event
    pub time: String,
    /// Is this a Vanity House release?
    pub vanity_house: bool,
    /// Is this release bookmarked?
    pub is_bookmarked: bool,
    /// Tags
    pub tags: Vec<String>,
    /// Release credits
    /// Artists, composer etc
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
            record_label: "Test Label".to_owned(),
            catalogue_number: "TEST-001".to_owned(),
            category_id: 0,
            category_name: "Music".to_owned(),
            time: "2020-01-01 00:00:00".to_owned(),
            vanity_house: false,
            is_bookmarked: false,
            music_info: None,
            tags: vec!["rock".to_owned()],
            wiki_body: "Test wiki body".to_owned(),
            bb_body: None,
            wiki_image: "https://example.com/image.jpg".to_owned(),
            release_type: 1,
        }
    }
}
