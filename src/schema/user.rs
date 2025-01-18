use serde::Deserialize;

/// Response for the `user` action
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Username
    pub username: String,
    /// Avatar image URL
    pub avatar: String,
    /// Are they a friend?
    pub is_friend: bool,
    /// Profile body formatted as HTML
    pub profile_text: String,
    /// Profile body formatted as BB code
    /// *RED only*
    pub bb_profile_text: Option<String>,
    /// Stats
    pub stats: Stats,
    /// Ranks
    pub ranks: Ranks,
    /// Personal
    pub personal: Personal,
    /// Community
    pub community: Community,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Join date
    pub joined_date: String,
    /// Last access date
    pub last_access: String,
    /// Bytes uploaded
    pub uploaded: u64,
    /// Bytes downloaded
    pub downloaded: u64,
    /// Ratio
    pub ratio: f32,
    /// Ratio required to maintain member level
    pub required_ratio: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ranks {
    /// Percentile of data uploaded
    pub uploaded: f32,
    /// Percentile of data downloaded
    pub downloaded: f32,
    /// Percentile of uploads
    pub uploads: f32,
    /// Percentile of requests filled
    pub requests: f32,
    /// Percentile of bounty spent
    pub bounty: f32,
    /// Percentile of posts made
    pub posts: f32,
    /// Percentile of artists added
    pub artists: f32,
    /// Overall percentile
    pub overall: f32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    /// Class
    pub class: String,
    /// Parnoia number
    pub paranoia: u8,
    /// Parnoia level
    pub paranoia_text: String,
    /// Have they donated?
    pub donor: bool,
    /// Have they been warned?
    pub warned: bool,
    /// Are they enabled?
    pub enabled: bool,
    /// Passkey
    pub passkey: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    /// Number of posts made
    pub posts: u32,
    /// Number of torrent comments
    pub torrent_comments: u32,
    /// Number of collages
    pub collages_started: u32,
    /// Number of collages contributed to
    pub collages_contrib: u32,
    /// Number of requests filled
    pub requests_filled: u32,
    /// Number of requests voted on
    pub requests_voted: u32,
    /// Number of perfect FLACs uploaded
    pub perfect_flacs: u32,
    /// Number of torrents uploaded
    pub uploaded: u32,
    /// Number of unique groups uploaded to
    pub groups: u32,
    /// Number of seeding torrents
    pub seeding: u32,
    /// Number of leeching torrents
    pub leeching: u32,
    /// Number of snatched torrents
    pub snatched: u32,
    /// Number of users invited
    pub invited: u32,
}
