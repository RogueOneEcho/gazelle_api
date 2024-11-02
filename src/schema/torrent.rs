use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct Torrent {
    pub id: i64,
    pub media: String,
    pub format: String,
    pub encoding: String,
    pub remastered: bool,
    pub remaster_year: Option<i64>,
    pub remaster_title: String,
    pub remaster_record_label: String,
    pub remaster_catalogue_number: String,
    pub scene: bool,
    pub has_log: bool,
    pub has_cue: bool,
    pub log_score: i64,
    pub file_count: i64,
    pub size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub snatched: i64,
    #[serde(rename = "has_snatched")]
    pub has_snatched: Option<bool>,
    pub trumpable: Option<bool>,
    pub lossy_web_approved: Option<bool>,
    pub lossy_master_approved: Option<bool>,
    #[serde(skip)]
    #[allow(clippy::struct_field_names)]
    pub free_torrent: Option<bool>,
    pub is_neutralleech: Option<bool>,
    pub is_freeload: Option<bool>,
    pub reported: bool,
    pub time: String,
    pub description: String,
    pub file_list: String,
    pub file_path: String,
    pub user_id: i64,
    pub username: String,
}

impl Torrent {
    pub fn get_flacs(&self) -> Vec<PathBuf> {
        Regex::new(r"([^|]+\.flac)\{\{\{\d+\}\}\}(?:\|\|\|)?")
            .expect("Regex should compile")
            .captures_iter(&self.file_list)
            .map(|cap| PathBuf::from(&cap[1]))
            .collect()
    }
}
