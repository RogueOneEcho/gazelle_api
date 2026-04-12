use serde::{Deserialize, Serialize};

/// Sort field for [`BrowseRequest`].
///
/// <https://github.com/OPSnet/Gazelle/blob/master/app/Search/Torrent.php>
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    /// Sort by release year
    Year,
    /// Sort by torrent ID (effectively upload time)
    Time,
    /// Sort by torrent size
    Size,
    /// Sort by seeder count
    Seeders,
    /// Sort by leecher count
    Leechers,
    /// Sort by snatch count
    Snatched,
    /// Random ordering
    Random,
}

impl OrderBy {
    /// Query parameter value for the Gazelle `order_by` parameter.
    #[must_use]
    pub fn as_query(&self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::Time => "time",
            Self::Size => "size",
            Self::Seeders => "seeders",
            Self::Leechers => "leechers",
            Self::Snatched => "snatched",
            Self::Random => "random",
        }
    }
}
