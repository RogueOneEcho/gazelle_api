use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

/// An edition of a release
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct Torrent {
    /// ID number
    pub id: u32,
    /// Media
    ///
    /// 0: `CD`
    /// 1: `DVD`
    /// 2: `Vinyl`
    /// 3: `Soundboard`
    /// 4: `SACD`
    /// 5: `DAT`
    /// 6: `Cassette`
    /// 7: `WEB`
    /// 8: `Blu-Ray` (Possibly `Blu-ray` on OPS)
    pub media: String,
    /// Format
    ///
    /// 0: `MP3`
    /// 1: `FLAC`
    /// 2: `AAC`
    /// 3: `AC3`
    /// 4: `DTS`
    ///
    /// *OPS may have others*
    pub format: String,
    /// Encoding
    ///
    /// 0: `192`
    /// 1: `APS (VBR)`
    /// 2: `V2 (VBR)`
    /// 3: `V1 (VBR)`
    /// 4: `256`
    /// 5: `APX (VBR)`
    /// 6: `V0 (VBR)`
    /// 7: `320`
    /// 8: `Lossless`
    /// 9: `24bit Lossless`
    /// 10: `Other`
    ///
    /// *OPS may have others*
    pub encoding: String,
    /// Has this release been confirmed?
    ///
    /// `remastered` is a deprecated term from the early days of Gazelle.
    /// If `false` then it will be displayed as an "Unconfirmed Release" and likely won't have a year
    pub remastered: bool,
    /// Edition year
    ///
    /// May be `0`
    pub remaster_year: Option<u16>,
    /// Edition title
    pub remaster_title: String,
    /// Edition record label
    pub remaster_record_label: String,
    /// Edition catalogue number
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
    /// *RED only*
    pub trumpable: Option<bool>,
    /// Is this an approved lossy web release?
    /// *RED only*
    pub lossy_web_approved: Option<bool>,
    /// Is this an approved lossy master release?
    /// *RED only*
    pub lossy_master_approved: Option<bool>,
    /// TBC
    /// *Inexplicably OPS returns this as an integer in a string*
    #[serde(skip)]
    #[allow(clippy::struct_field_names)]
    pub free_torrent: Option<bool>,
    /// Is this neutral leech?
    /// *RED only*
    pub is_neutralleech: Option<bool>,
    /// Is this freeload?
    /// *RED only*
    pub is_freeload: Option<bool>,
    /// Has this been reported?
    pub reported: bool,
    /// Time of last logged event
    pub time: String,
    /// Description formatted as BB code
    pub description: String,
    /// Encoded string of files and their sizes
    pub file_list: String,
    /// The name of the torrent directory
    pub file_path: String,
    /// ID of uploader
    pub user_id: u32,
    /// Username of uploader
    pub username: String,
}

impl Torrent {
    #[must_use]
    pub fn get_flacs(&self) -> Vec<PathBuf> {
        Regex::new(r"([^|]+\.flac)\{\{\{\d+\}\}\}(?:\|\|\|)?")
            .expect("Regex should compile")
            .captures_iter(&self.file_list)
            .map(|cap| PathBuf::from(&cap[1]))
            .collect()
    }
}

#[cfg(feature = "mock")]
impl Torrent {
    /// Create a mock Torrent for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            id: 456,
            media: "CD".to_owned(),
            format: "FLAC".to_owned(),
            encoding: "Lossless".to_owned(),
            remastered: true,
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
            file_list: "test.flac{{{100000}}}|||".to_owned(),
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
        let file_list = r"file1.flac{{{12345}}}|||file2.flac{{{67890}}}|||file with spaces.flac{{{54321}}}|||another_file.flac{{{98765}}}|||/path/to/file.flac{{{11111}}}|||C:\windows\path\file.flac{{{22222}}}|||Disc 1/01. track with period.flac{{{33333}}}|||Disc 1/02. track-with-dash.flac{{{44444}}}|||track_with_underscores.flac{{{55555}}}|||file_with_numbers_123.flac{{{66666}}}|||special&char#file.flac{{{77777}}}|||final_file.flac{{{88888}}}cover.jpg{{{123456}}}|||archive.zip{{{234567}}}|||executable.exe{{{345678}}}|||document.pdf{{{456789}}}|||presentation.pptx{{{567890}}}|||disc-image.iso{{{678901}}}|||compressed.tar.gz{{{789012}}}|||photo.png{{{890123}}}|||audio.mp3{{{901234}}}|||final.zip{{{912345}}}".to_owned();
        let torrent = Torrent {
            file_list,
            ..Torrent::default()
        };

        // Act
        let actual = torrent.get_flacs();

        // Assert
        let expected = vec![
            PathBuf::from("file1.flac"),
            PathBuf::from("file2.flac"),
            PathBuf::from("file with spaces.flac"),
            PathBuf::from("another_file.flac"),
            PathBuf::from("/path/to/file.flac"),
            PathBuf::from(r"C:\windows\path\file.flac"),
            PathBuf::from("Disc 1/01. track with period.flac"),
            PathBuf::from("Disc 1/02. track-with-dash.flac"),
            PathBuf::from("track_with_underscores.flac"),
            PathBuf::from("file_with_numbers_123.flac"),
            PathBuf::from("special&char#file.flac"),
            PathBuf::from("final_file.flac"),
        ];
        assert_eq!(actual, expected);
    }
}
