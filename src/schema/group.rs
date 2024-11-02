use serde::Deserialize;

use crate::MusicInfo;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub wiki_body: String,
    pub bb_body: Option<String>,
    pub wiki_image: String,
    pub id: u32,
    pub name: String,
    pub year: u16,
    pub record_label: String,
    pub catalogue_number: String,
    pub release_type: u8,
    pub category_id: u8,
    pub category_name: String,
    pub time: String,
    pub vanity_house: bool,
    pub is_bookmarked: bool,
    pub tags: Vec<String>,
    pub music_info: Option<MusicInfo>,
}
