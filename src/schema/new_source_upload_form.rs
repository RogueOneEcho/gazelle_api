use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_yaml::to_string as yaml_to_string;

/// Artist credit entry for new source uploads.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewSourceUploadArtist {
    /// Artist name.
    pub name: String,
    /// Artist role/importance index in Gazelle upload form.
    pub role: u8,
}

/// Edition metadata for new source uploads.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewSourceUploadEdition {
    /// Whether release-specific edition metadata is unknown.
    pub unknown_release: bool,
    /// Optional explicit remaster flag.
    pub remaster: Option<bool>,
    /// Edition year.
    pub year: u16,
    /// Edition title.
    pub title: String,
    /// Edition record label.
    pub record_label: String,
    /// Edition catalogue number.
    pub catalogue_number: String,
    /// Audio format.
    pub format: String,
    /// Audio bitrate/encoding.
    pub bitrate: String,
}

/// Form data for uploading a new source torrent and creating a new group.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewSourceUploadForm {
    /// Path to the torrent file to upload.
    pub path: PathBuf,
    /// ID number of the category.
    pub category_id: u8,
    /// Group title.
    pub title: String,
    /// Original release year.
    pub year: u16,
    /// Release type index.
    pub release_type: u8,
    /// Media type.
    pub media: String,
    /// Tags for the release.
    pub tags: Vec<String>,
    /// Album description (group description).
    pub album_desc: String,
    /// Upload description (torrent description).
    pub release_desc: String,
    /// Optional request ID this upload is filling.
    pub request_id: Option<u32>,
    /// Optional image URL.
    pub image: Option<String>,
    /// Edition metadata.
    pub edition: NewSourceUploadEdition,
    /// Artist credits.
    pub artists: Vec<NewSourceUploadArtist>,
}

impl NewSourceUploadForm {
    /// Convert this payload into text field pairs.
    ///
    /// This helper keeps multipart field mapping testable without relying on private reqwest types.
    #[must_use]
    pub fn to_text_fields(&self) -> Vec<(String, String)> {
        let mut fields = vec![
            ("type".to_owned(), self.category_id.to_string()),
            ("title".to_owned(), self.title.clone()),
            ("year".to_owned(), self.year.to_string()),
            ("releasetype".to_owned(), self.release_type.to_string()),
            ("format".to_owned(), self.edition.format.clone()),
            ("bitrate".to_owned(), self.edition.bitrate.clone()),
            ("media".to_owned(), self.media.clone()),
            ("tags".to_owned(), self.tags.join(",")),
            ("album_desc".to_owned(), self.album_desc.clone()),
            ("release_desc".to_owned(), self.release_desc.clone()),
            (
                "unknown".to_owned(),
                if self.edition.unknown_release {
                    "1".to_owned()
                } else {
                    "0".to_owned()
                },
            ),
        ];
        if let Some(request_id) = self.request_id {
            fields.push(("requestid".to_owned(), request_id.to_string()));
        }
        if let Some(image) = &self.image {
            fields.push(("image".to_owned(), image.clone()));
        }
        if let Some(remaster) = self.edition.remaster {
            fields.push((
                "remaster".to_owned(),
                if remaster {
                    "1".to_owned()
                } else {
                    "0".to_owned()
                },
            ));
        }
        if !self.edition.unknown_release {
            fields.push(("remaster_year".to_owned(), self.edition.year.to_string()));
            fields.push(("remaster_title".to_owned(), self.edition.title.clone()));
            fields.push((
                "remaster_record_label".to_owned(),
                self.edition.record_label.clone(),
            ));
            fields.push((
                "remaster_catalogue_number".to_owned(),
                self.edition.catalogue_number.clone(),
            ));
        }
        for artist in &self.artists {
            fields.push(("artists[]".to_owned(), artist.name.clone()));
            fields.push(("importance[]".to_owned(), artist.role.to_string()));
        }
        fields
    }

    /// Convert to a multipart form for the upload API request.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_form(self) -> Result<Form, Error> {
        let mut file = File::open(&self.path)?;
        let mut buffer = Vec::new();
        let _size = file.read_to_end(&mut buffer)?;
        let filename = self
            .path
            .file_name()
            .expect("file should have a name")
            .to_string_lossy()
            .to_string();
        let torrent_part = Part::bytes(buffer).file_name(filename);
        let mut form = Form::new().part("file_input", torrent_part);
        for (key, value) in self.to_text_fields() {
            form = form.text(key, value);
        }
        Ok(form)
    }
}

#[cfg(any(test, feature = "mock"))]
impl NewSourceUploadForm {
    /// Create a mock form for testing.
    #[must_use]
    pub fn mock() -> Self {
        Self {
            path: PathBuf::from("/tmp/example.torrent"),
            category_id: 0,
            title: "Example Album".to_owned(),
            year: 2024,
            release_type: 1,
            media: "WEB".to_owned(),
            tags: vec!["electronic".to_owned()],
            album_desc: String::new(),
            release_desc: String::new(),
            request_id: None,
            image: None,
            edition: NewSourceUploadEdition {
                unknown_release: true,
                remaster: None,
                year: 0,
                title: String::new(),
                record_label: String::new(),
                catalogue_number: String::new(),
                format: "FLAC".to_owned(),
                bitrate: "Lossless".to_owned(),
            },
            artists: vec![NewSourceUploadArtist {
                name: "Artist".to_owned(),
                role: 1,
            }],
        }
    }
}

impl Display for NewSourceUploadForm {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let output = if let Ok(yaml) = yaml_to_string(self) {
            yaml
        } else {
            format!("{self:?}")
        };
        output.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs};

    #[test]
    fn to_text_fields_contains_required_upload_keys() {
        // Arrange
        let form = NewSourceUploadForm {
            tags: vec!["electronic".to_owned(), "ambient".to_owned()],
            album_desc: "Album description".to_owned(),
            release_desc: "Release description".to_owned(),
            request_id: Some(123_456),
            image: Some("https://example.com/cover.jpg".to_owned()),
            edition: NewSourceUploadEdition {
                unknown_release: false,
                remaster: Some(true),
                year: 2024,
                title: "Digital".to_owned(),
                record_label: "Label".to_owned(),
                catalogue_number: "CAT-001".to_owned(),
                format: "FLAC".to_owned(),
                bitrate: "Lossless".to_owned(),
            },
            ..NewSourceUploadForm::mock()
        };

        // Act
        let fields = form.to_text_fields();
        let keys: Vec<String> = fields.into_iter().map(|(k, _)| k).collect();

        // Assert
        for required in [
            "type",
            "title",
            "year",
            "releasetype",
            "format",
            "bitrate",
            "media",
            "tags",
            "album_desc",
            "release_desc",
            "requestid",
            "image",
            "unknown",
            "remaster",
            "remaster_year",
            "remaster_title",
            "remaster_record_label",
            "remaster_catalogue_number",
            "artists[]",
            "importance[]",
        ] {
            assert!(
                keys.contains(&required.to_owned()),
                "missing key: {required}"
            );
        }
    }

    #[test]
    fn to_form_reads_torrent_file_and_builds_multipart() {
        // Arrange
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("should get unix time")
            .as_nanos();
        let path = env::temp_dir().join(format!("gazelle_api_new_source_{now}.torrent"));
        fs::write(&path, [1, 2, 3, 4]).expect("should write temp torrent");
        let form = NewSourceUploadForm {
            path: path.clone(),
            ..NewSourceUploadForm::mock()
        };

        // Act
        let result = form.to_form();

        // Assert
        assert!(result.is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn to_text_fields_maps_unknown_release_and_omits_edition_fields_when_unknown() {
        // Arrange
        let known_release = NewSourceUploadForm {
            title: "Known Release".to_owned(),
            edition: NewSourceUploadEdition {
                unknown_release: false,
                remaster: None,
                year: 2024,
                title: "Digital".to_owned(),
                record_label: "Label".to_owned(),
                catalogue_number: "CAT-001".to_owned(),
                format: "FLAC".to_owned(),
                bitrate: "Lossless".to_owned(),
            },
            ..NewSourceUploadForm::mock()
        };
        let unknown_release = NewSourceUploadForm {
            edition: NewSourceUploadEdition {
                unknown_release: true,
                ..known_release.edition.clone()
            },
            ..known_release.clone()
        };

        // Act
        let known_fields: HashMap<String, String> =
            known_release.to_text_fields().into_iter().collect();
        let unknown_fields: HashMap<String, String> =
            unknown_release.to_text_fields().into_iter().collect();

        // Assert
        assert_eq!(known_fields.get("unknown").map(String::as_str), Some("0"));
        assert!(known_fields.contains_key("remaster_year"));
        assert!(known_fields.contains_key("remaster_title"));
        assert!(known_fields.contains_key("remaster_record_label"));
        assert!(known_fields.contains_key("remaster_catalogue_number"));
        assert_eq!(unknown_fields.get("unknown").map(String::as_str), Some("1"));
        assert!(!unknown_fields.contains_key("remaster_year"));
        assert!(!unknown_fields.contains_key("remaster_title"));
        assert!(!unknown_fields.contains_key("remaster_record_label"));
        assert!(!unknown_fields.contains_key("remaster_catalogue_number"));
    }

    #[test]
    fn to_text_fields_maps_remaster_when_specified() {
        // Arrange
        let with_remaster = NewSourceUploadForm {
            title: "Remaster".to_owned(),
            edition: NewSourceUploadEdition {
                unknown_release: false,
                remaster: Some(true),
                year: 2024,
                title: "Digital".to_owned(),
                record_label: "Label".to_owned(),
                catalogue_number: "CAT-001".to_owned(),
                format: "FLAC".to_owned(),
                bitrate: "Lossless".to_owned(),
            },
            ..NewSourceUploadForm::mock()
        };
        let without_remaster = NewSourceUploadForm {
            edition: NewSourceUploadEdition {
                remaster: None,
                ..with_remaster.edition.clone()
            },
            ..with_remaster.clone()
        };

        // Act
        let with_remaster_fields: HashMap<String, String> =
            with_remaster.to_text_fields().into_iter().collect();
        let without_remaster_fields: HashMap<String, String> =
            without_remaster.to_text_fields().into_iter().collect();

        // Assert
        assert_eq!(
            with_remaster_fields.get("remaster").map(String::as_str),
            Some("1")
        );
        assert!(!without_remaster_fields.contains_key("remaster"));
    }

    #[test]
    fn to_text_fields_maps_numeric_release_type() {
        // Arrange
        let form = NewSourceUploadForm {
            title: "Release Type".to_owned(),
            release_type: 21,
            ..NewSourceUploadForm::mock()
        };

        // Act
        let fields: HashMap<String, String> = form.to_text_fields().into_iter().collect();

        // Assert
        assert_eq!(fields.get("releasetype").map(String::as_str), Some("21"));
    }

    #[test]
    fn to_text_fields_includes_request_id_when_present() {
        // Arrange
        let with_request_id = NewSourceUploadForm {
            request_id: Some(364_781),
            ..NewSourceUploadForm::mock()
        };
        let without_request_id = NewSourceUploadForm {
            request_id: None,
            ..with_request_id.clone()
        };

        // Act
        let with_request_id_fields: HashMap<String, String> =
            with_request_id.to_text_fields().into_iter().collect();
        let without_request_id_fields: HashMap<String, String> =
            without_request_id.to_text_fields().into_iter().collect();

        // Assert
        assert_eq!(
            with_request_id_fields.get("requestid").map(String::as_str),
            Some("364781")
        );
        assert!(!without_request_id_fields.contains_key("requestid"));
    }
}
