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
    /// Public collages containing this release.
    ///
    /// *RED only*
    #[serde(default)]
    pub collages: Option<Vec<Collage>>,
    /// Personal collages containing this release.
    ///
    /// *RED only*
    #[serde(default)]
    pub personal_collages: Option<Vec<Collage>>,
    /// Number of comments.
    ///
    /// *RED torrentgroup only*
    #[serde(default, deserialize_with = "string_or_u32_opt")]
    pub num_comments: Option<u32>,
    /// Proxied cover image URL.
    ///
    /// *OPS only*
    #[serde(default)]
    pub proxy_image: Option<String>,
    /// Release type display name.
    ///
    /// *OPS only*
    #[serde(default)]
    pub release_type_name: Option<String>,
    /// Album info formatted as BB code.
    ///
    /// - *OPS only*: uses `wikiBBcode` key; RED uses [`Group::bb_body`] instead
    #[serde(
        default,
        rename = "wikiBBcode",
        deserialize_with = "decode_entities_opt"
    )]
    pub wiki_bb_code: Option<String>,
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
            collages: None,
            personal_collages: None,
            num_comments: None,
            proxy_image: None,
            release_type_name: None,
            wiki_bb_code: None,
        }
    }
}

/// Deserialize an `Option<u32>` from a number, a string, or null.
///
/// - RED returns `numComments` as a string (e.g. `"7"`)
fn string_or_u32_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u32>, D::Error> {
    struct StringOrU32OptVisitor;

    impl Visitor<'_> for StringOrU32OptVisitor {
        type Value = Option<u32>;

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("a u32, a string containing a u32, or null")
        }

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            u32::try_from(value).map(Some).map_err(DeError::custom)
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map(Some).map_err(DeError::custom)
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringOrU32OptVisitor)
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
