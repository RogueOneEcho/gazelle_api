use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

/// An edition of a release
/// <https://github.com/OPSnet/Gazelle/blob/master/docs/07-API.md#torrent>
#[derive(Debug, Default, Deserialize)]
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
