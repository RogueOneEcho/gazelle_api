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
    /// Profile body formatted as BB code.
    ///
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

/// User statistics
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

/// User ranking percentiles
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

/// User personal information
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    /// Class
    pub class: String,
    /// Paranoia number
    pub paranoia: u8,
    /// Paranoia level
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

/// User community activity
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod tests {
    use super::*;

    const OPS_RESPONSE: &str = include_str!("../tests/fixtures/user_response_ops.json");
    const RED_RESPONSE: &str = include_str!("../tests/fixtures/user_response_red.json");

    #[test]
    fn deserialize_ops_user_response() {
        // Arrange & Act
        let user: User = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - OPS lacks bb_profile_text
        assert!(user.bb_profile_text.is_none());

        // Assert - Core fields
        assert_eq!(user.username, "TestUser");
        assert!(!user.is_friend);
        assert!(user.profile_text.is_empty());
    }

    #[test]
    fn deserialize_red_user_response() {
        // Arrange & Act
        let user: User = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - RED has bb_profile_text
        assert!(user.bb_profile_text.is_some());
        assert!(user.bb_profile_text.as_ref().unwrap().contains("Developer"));

        // Assert - Core fields
        assert_eq!(user.username, "TestUser");
        assert!(!user.is_friend);
        assert!(user.profile_text.contains("Developer"));
    }

    #[test]
    fn deserialize_user_stats() {
        // Arrange & Act
        let user: User = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - Stats fields
        assert_eq!(user.stats.joined_date, "2023-04-13 03:49:47");
        assert_eq!(user.stats.uploaded, 1_207_152_087_233);
        assert_eq!(user.stats.downloaded, 373_638_950_466);
        assert!(user.stats.ratio > 3.0);
        assert!(user.stats.required_ratio < 1.0);
    }

    #[test]
    fn deserialize_user_ranks() {
        // Arrange & Act
        let user: User = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - Ranks fields
        assert_eq!(user.ranks.uploaded, 95.0);
        assert_eq!(user.ranks.downloaded, 93.0);
        assert_eq!(user.ranks.uploads, 94.0);
        assert_eq!(user.ranks.overall, 36.0);
    }

    #[test]
    fn deserialize_user_personal() {
        // Arrange & Act
        let user: User = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - Personal fields
        assert_eq!(user.personal.class, "Torrent Master");
        assert_eq!(user.personal.paranoia, 34);
        assert_eq!(user.personal.paranoia_text, "Very high");
        assert!(!user.personal.donor);
        assert!(!user.personal.warned);
        assert!(user.personal.enabled);
    }

    #[test]
    fn deserialize_user_community() {
        // Arrange & Act
        let user: User = serde_json::from_str(OPS_RESPONSE).unwrap();

        // Assert - Community fields
        assert_eq!(user.community.posts, 9);
        assert_eq!(user.community.perfect_flacs, 81);
        assert_eq!(user.community.uploaded, 578);
        assert_eq!(user.community.groups, 287);
        assert_eq!(user.community.seeding, 1127);
        assert_eq!(user.community.snatched, 982);
    }

    #[test]
    fn deserialize_red_user_community() {
        // Arrange & Act
        let user: User = serde_json::from_str(RED_RESPONSE).unwrap();

        // Assert - RED community has different values
        assert_eq!(user.community.posts, 63);
        assert_eq!(user.community.perfect_flacs, 246);
        assert_eq!(user.community.seeding, 2660);
        assert_eq!(user.community.snatched, 4789);
    }
}
