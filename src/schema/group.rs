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
    #[serde(deserialize_with = "decode_entities")]
    pub wiki_body: String,
    /// Album info formatted as BB code
    #[serde(default, deserialize_with = "decode_entities_opt")]
    pub bb_body: Option<String>,
    /// Cover image URL
    pub wiki_image: String,
    /// ID number
    pub id: u32,
    /// Release Name
    #[serde(deserialize_with = "decode_entities")]
    pub name: String,
    /// Release Year
    pub year: u16,
    /// Record label
    ///
    /// *RED only*
    #[serde(default, deserialize_with = "decode_entities_opt")]
    pub record_label: Option<String>,
    /// Release catalogue number
    ///
    /// *RED only*
    #[serde(default, deserialize_with = "decode_entities_opt")]
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

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn group_text_fields_decoded() {
        let json = r#"{
            "wikiBody": "Some &lt;b&gt;text&lt;/b&gt;",
            "bbBody": "[b]bb&amp;text[/b]",
            "wikiImage": "",
            "id": 1,
            "name": "Best &amp; Greatest",
            "year": 2020,
            "recordLabel": "Acme &amp;",
            "catalogueNumber": "XYZ&#039;1",
            "releaseType": 1,
            "categoryId": 1,
            "categoryName": "Music",
            "time": "2020-01-01 00:00:00",
            "vanityHouse": false,
            "isBookmarked": false,
            "tags": [],
            "musicInfo": null
        }"#;
        let group: Group = json_from_str(json).expect("fixture should deserialize");
        assert_eq!(group.name, "Best & Greatest");
        assert_eq!(group.wiki_body, "Some <b>text</b>");
        assert_eq!(group.bb_body.as_deref(), Some("[b]bb&text[/b]"));
        assert_eq!(group.record_label.as_deref(), Some("Acme &"));
        assert_eq!(group.catalogue_number.as_deref(), Some("XYZ'1"));
    }
}
