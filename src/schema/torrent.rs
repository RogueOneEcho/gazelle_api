use crate::prelude::*;

/// An edition of a release.
///
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[expect(clippy::struct_excessive_bools)]
pub struct Torrent {
    /// ID number
    pub id: u32,
    /// Media
    pub media: Media,
    /// Format
    pub format: Format,
    /// Quality
    ///
    /// Referred to as `encoding` in the API response.
    pub encoding: Quality,
    /// Has this release been confirmed?
    ///
    /// `remastered` is a deprecated term from the early days of Gazelle.
    /// If `false` then it will be displayed as an "Unconfirmed Release" and likely won't have a year
    ///
    /// *RED only*
    pub remastered: Option<bool>,
    /// Edition year
    ///
    /// May be `0`
    pub remaster_year: Option<u16>,
    /// Edition title
    #[serde(deserialize_with = "decode_entities")]
    pub remaster_title: String,
    /// Edition record label
    #[serde(deserialize_with = "decode_entities")]
    pub remaster_record_label: String,
    /// Edition catalogue number
    #[serde(deserialize_with = "decode_entities")]
    pub remaster_catalogue_number: String,
    /// Is this a scene release?
    pub scene: bool,
    /// Is there a log?
    pub has_log: bool,
    /// Is there a cue?
    pub has_cue: bool,
    /// The log score
    pub log_score: i32,
    /// Number of files in the torrent
    pub file_count: u32,
    /// Total size of the files in bytes
    pub size: u64,
    /// Number of seeders
    pub seeders: u32,
    /// Number of leechers
    pub leechers: u32,
    /// Number of times snatched
    pub snatched: u32,
    /// Has the user snatched the torrent?
    #[serde(rename = "has_snatched")]
    pub has_snatched: Option<bool>,
    /// Is this trumpable?
    ///
    /// *RED only*
    pub trumpable: Option<bool>,
    /// Is this an approved lossy web release?
    ///
    /// *RED only*
    pub lossy_web_approved: Option<bool>,
    /// Is this an approved lossy master release?
    ///
    /// *RED only*
    pub lossy_master_approved: Option<bool>,
    /// Is this a freeleech torrent?
    ///
    /// *Skipped because OPS returns this as an integer in a string*
    #[serde(skip)]
    pub free_torrent: Option<bool>,
    /// Is this neutral leech?
    ///
    /// *RED only*
    pub is_neutralleech: Option<bool>,
    /// Is this freeload?
    ///
    /// *RED only*
    pub is_freeload: Option<bool>,
    /// Has this been reported?
    pub reported: bool,
    /// Time of last logged event
    pub time: String,
    /// Description formatted as BB code
    #[serde(deserialize_with = "decode_entities")]
    pub description: String,
    /// Raw `name{{{size}}}|||...` file list.
    ///
    /// - File names are HTML-entity-encoded as returned by Gazelle.
    /// - Use [`parse_file_list`] to get decoded [`FileItem`].
    pub file_list: String,
    /// The name of the torrent directory
    #[serde(deserialize_with = "decode_entities")]
    pub file_path: String,
    /// ID of uploader
    pub user_id: u32,
    /// Username of uploader
    #[serde(deserialize_with = "decode_entities")]
    pub username: String,
}

impl Torrent {
    /// Extract FLAC file paths from the encoded file list
    #[must_use]
    pub fn get_flacs(&self) -> Vec<PathBuf> {
        self.get_files()
            .into_iter()
            .filter(|item| item.name.to_lowercase().ends_with(".flac"))
            .map(|item| PathBuf::from(item.name))
            .collect()
    }

    /// Parse the file list into a vec of [`FileItem`] entries, sorted by filename.
    #[must_use]
    pub fn get_files(&self) -> Vec<FileItem> {
        let mut files = parse_file_list(&self.file_list);
        files.sort_by(|a, b| a.name.cmp(&b.name));
        files
    }
}

#[cfg(feature = "mock")]
impl Torrent {
    /// Create a mock Torrent for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            id: 456,
            media: Media::CD,
            format: Format::FLAC,
            encoding: Quality::Lossless,
            remastered: Some(true),
            remaster_year: Some(2020),
            remaster_title: String::new(),
            remaster_record_label: "Test Label".to_owned(),
            remaster_catalogue_number: "TEST-001".to_owned(),
            scene: false,
            has_log: true,
            has_cue: true,
            log_score: 100,
            file_count: 10,
            size: 500_000_000,
            seeders: 50,
            leechers: 2,
            snatched: 100,
            has_snatched: None,
            trumpable: None,
            lossy_web_approved: None,
            lossy_master_approved: None,
            free_torrent: None,
            is_neutralleech: None,
            is_freeload: None,
            reported: false,
            time: "2020-01-01 00:00:00".to_owned(),
            description: "Test description".to_owned(),
            file_list: "test.flac{{{100000}}}".to_owned(),
            file_path: "Test Album (2020) [FLAC]".to_owned(),
            user_id: 1,
            username: "uploader".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_flacs() {
        // Arrange
        let file_list = r"file1.flac{{{12345}}}|||file2.flac{{{67890}}}|||file with spaces.flac{{{54321}}}|||another_file.flac{{{98765}}}|||/path/to/file.flac{{{11111}}}|||C:\windows\path\file.flac{{{22222}}}|||Disc 1/01. track with period.flac{{{33333}}}|||Disc 1/02. track-with-dash.flac{{{44444}}}|||track_with_underscores.flac{{{55555}}}|||file_with_numbers_123.flac{{{66666}}}|||special&char#file.flac{{{77777}}}|||final_file.flac{{{88888}}}|||cover.jpg{{{123456}}}|||archive.zip{{{234567}}}|||executable.exe{{{345678}}}|||document.pdf{{{456789}}}|||presentation.pptx{{{567890}}}|||disc-image.iso{{{678901}}}|||compressed.tar.gz{{{789012}}}|||photo.png{{{890123}}}|||audio.mp3{{{901234}}}|||final.zip{{{912345}}}".to_owned();
        let torrent = Torrent {
            file_list,
            ..Torrent::default()
        };

        // Act
        let actual = torrent.get_flacs();

        // Assert
        let expected = vec![
            PathBuf::from("/path/to/file.flac"),
            PathBuf::from(r"C:\windows\path\file.flac"),
            PathBuf::from("Disc 1/01. track with period.flac"),
            PathBuf::from("Disc 1/02. track-with-dash.flac"),
            PathBuf::from("another_file.flac"),
            PathBuf::from("file with spaces.flac"),
            PathBuf::from("file1.flac"),
            PathBuf::from("file2.flac"),
            PathBuf::from("file_with_numbers_123.flac"),
            PathBuf::from("final_file.flac"),
            PathBuf::from("special&char#file.flac"),
            PathBuf::from("track_with_underscores.flac"),
        ];
        assert_eq!(actual, expected);
    }

    #[cfg(feature = "mock")]
    mod parse_file_list_tests {
        use super::*;

        #[test]
        fn torrent_get_files_decodes_entities() {
            let torrent = Torrent {
                file_list:
                    "Artist &amp; Title.flac{{{12345}}}|||cover &#039;art&#039;.jpg{{{500}}}"
                        .to_owned(),
                ..Torrent::mock()
            };
            let output = torrent.get_files();
            assert_eq!(output.len(), 2);
            assert!(output.contains(&FileItem {
                name: "Artist & Title.flac".to_owned(),
                size: 12_345,
            }));
            assert!(output.contains(&FileItem {
                name: "cover 'art'.jpg".to_owned(),
                size: 500,
            }));
        }

        #[test]
        fn torrent_get_files() {
            // Arrange
            let torrent = Torrent {
                file_list: "01 - Track.flac{{{12345678}}}|||cover.jpg{{{98765}}}".to_owned(),
                ..Torrent::mock()
            };

            // Act
            let output = torrent.get_files();

            // Assert
            assert_eq!(output.len(), 2);
            assert!(output.contains(&FileItem {
                name: "01 - Track.flac".to_owned(),
                size: 12_345_678,
            }));
            assert!(output.contains(&FileItem {
                name: "cover.jpg".to_owned(),
                size: 98_765,
            }));
        }

        #[test]
        fn torrent_get_files_single_item() {
            let torrent = Torrent {
                file_list: "single.flac{{{100}}}".to_owned(),
                ..Torrent::mock()
            };
            let output = torrent.get_files();
            assert_eq!(output.len(), 1);
            assert!(output.contains(&FileItem {
                name: "single.flac".to_owned(),
                size: 100,
            }));
        }

        #[test]
        fn torrent_get_files_empty() {
            let torrent = Torrent {
                file_list: String::new(),
                ..Torrent::mock()
            };
            assert!(torrent.get_files().is_empty());
        }

        #[test]
        fn torrent_get_files_malformed_entries_skipped() {
            let torrent = Torrent {
                file_list: "goodfile.flac{{{100}}}|||badentry".to_owned(),
                ..Torrent::mock()
            };
            let output = torrent.get_files();
            assert_eq!(output.len(), 1);
            assert!(output.contains(&FileItem {
                name: "goodfile.flac".to_owned(),
                size: 100,
            }));
        }
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn torrent_text_fields_decoded() {
        let json = r#"{
            "id": 1,
            "media": "WEB",
            "format": "FLAC",
            "encoding": "Lossless",
            "remastered": true,
            "remasterYear": 2020,
            "remasterTitle": "Deluxe &amp; Expanded",
            "remasterRecordLabel": "Acme &amp; Co",
            "remasterCatalogueNumber": "ABC&#039;123",
            "scene": false,
            "hasLog": false,
            "hasCue": false,
            "logScore": 0,
            "fileCount": 1,
            "size": 1,
            "seeders": 0,
            "leechers": 0,
            "snatched": 0,
            "has_snatched": false,
            "trumpable": null,
            "lossyWebApproved": null,
            "lossyMasterApproved": null,
            "isNeutralleech": null,
            "isFreeload": null,
            "reported": false,
            "time": "2020-01-01 00:00:00",
            "description": "Notes &amp; info",
            "fileList": "",
            "filePath": "Artist &amp; Title",
            "userId": 1,
            "username": "DJ &amp; MC"
        }"#;
        let torrent: Torrent = json_from_str(json).expect("fixture should deserialize");
        assert_eq!(torrent.remaster_title, "Deluxe & Expanded");
        assert_eq!(torrent.remaster_record_label, "Acme & Co");
        assert_eq!(torrent.remaster_catalogue_number, "ABC'123");
        assert_eq!(torrent.description, "Notes & info");
        assert_eq!(torrent.file_path, "Artist & Title");
        assert_eq!(torrent.username, "DJ & MC");
    }
}
