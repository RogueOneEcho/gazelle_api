use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub avatar: String,
    pub is_friend: bool,
    pub profile_text: String,
    pub bb_profile_text: Option<String>,
    pub stats: Stats,
    pub ranks: Ranks,
    pub personal: Personal,
    pub community: Community,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub joined_date: String,
    pub last_access: String,
    pub uploaded: u64,
    pub downloaded: u64,
    pub ratio: f32,
    pub required_ratio: f32,
}

#[derive(Debug, Deserialize)]
pub struct Ranks {
    pub uploaded: f32,
    pub downloaded: f32,
    pub uploads: f32,
    pub requests: f32,
    pub bounty: f32,
    pub posts: f32,
    pub artists: f32,
    pub overall: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    pub class: String,
    pub paranoia: u8,
    pub paranoia_text: String,
    pub donor: bool,
    pub warned: bool,
    pub enabled: bool,
    pub passkey: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub posts: u32,
    pub torrent_comments: u32,
    pub collages_started: u32,
    pub collages_contrib: u32,
    pub requests_filled: u32,
    pub requests_voted: u32,
    pub perfect_flacs: u32,
    pub uploaded: u32,
    pub groups: u32,
    pub seeding: u32,
    pub leeching: u32,
    pub snatched: u32,
    pub invited: u32,
}
