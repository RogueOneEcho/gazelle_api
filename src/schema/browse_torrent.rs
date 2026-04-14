use crate::{Credit, Format, Media, Quality, Torrent};
use serde::Deserialize;

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

#[cfg(feature = "mock")]
impl BrowseTorrent {
    /// Create a mock [`BrowseTorrent`] for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            torrent_id: 456,
            edition_id: 1,
            artists: vec![Credit {
                id: 1,
                name: "Test Artist".to_owned(),
            }],
            media: Media::CD,
            format: Format::FLAC,
            encoding: Quality::Lossless24,
            remastered: Some(true),
            remaster_year: Some(2020),
            remaster_record_label: Some("Test Label".to_owned()),
            remaster_catalogue_number: "TEST-001".to_owned(),
            remaster_title: String::new(),
            has_log: true,
            log_score: 100,
            has_cue: true,
            scene: false,
            vanity_house: false,
            file_count: 10,
            time: "2020-01-01 00:00:00".to_owned(),
            size: 500_000_000,
            snatches: 100,
            seeders: 50,
            leechers: 2,
            is_freeleech: false,
            is_neutral_leech: false,
            is_personal_freeleech: false,
            can_use_token: true,
            has_snatched: false,
            leech_status: None,
            is_freeload: None,
            trumpable: None,
        }
    }
}
