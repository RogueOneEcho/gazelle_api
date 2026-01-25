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

#[cfg(feature = "mock")]
impl User {
    /// Create a mock User for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            username: "testuser".to_owned(),
            avatar: "https://example.com/avatar.png".to_owned(),
            is_friend: false,
            profile_text: "Test profile".to_owned(),
            bb_profile_text: None,
            stats: Stats::mock(),
            ranks: Ranks::mock(),
            personal: Personal::mock(),
            community: Community::mock(),
        }
    }
}

#[cfg(feature = "mock")]
impl Stats {
    /// Create a mock Stats for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            joined_date: "2020-01-01 00:00:00".to_owned(),
            last_access: "2024-01-01 00:00:00".to_owned(),
            uploaded: 1_000_000_000,
            downloaded: 500_000_000,
            ratio: 2.0,
            required_ratio: 0.5,
        }
    }
}

#[cfg(feature = "mock")]
impl Ranks {
    /// Create a mock Ranks for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            uploaded: 90.0,
            downloaded: 50.0,
            uploads: 80.0,
            requests: 10.0,
            bounty: 5.0,
            posts: 20.0,
            artists: 15.0,
            overall: 60.0,
        }
    }
}

#[cfg(feature = "mock")]
impl Personal {
    /// Create a mock Personal for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            class: "Member".to_owned(),
            paranoia: 0,
            paranoia_text: "Off".to_owned(),
            donor: false,
            warned: false,
            enabled: true,
            passkey: "testpasskey".to_owned(),
        }
    }
}

#[cfg(feature = "mock")]
impl Community {
    /// Create a mock Community for testing
    #[must_use]
    pub fn mock() -> Self {
        Self {
            posts: 100,
            torrent_comments: 50,
            collages_started: 2,
            collages_contrib: 5,
            requests_filled: 10,
            requests_voted: 20,
            perfect_flacs: 30,
            uploaded: 40,
            groups: 35,
            seeding: 100,
            leeching: 2,
            snatched: 200,
            invited: 5,
        }
    }
}
