use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

/// Release type of a [`Group`].
///
/// Non-music categories use [`ReleaseType::NonMusic`] which are stored as in the database as `0`
/// but returned by the API as `""`.
///
/// - <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L582-L595>
/// - <https://github.com/OPSnet/Gazelle/blob/be7fae7c70028db381a5738bba6277d3b6812aa8/app/Json/TGroup.php#L26>
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ReleaseType {
    /// Album
    Album,
    /// Soundtrack
    Soundtrack,
    /// EP
    EP,
    /// Anthology
    Anthology,
    /// Compilation
    Compilation,
    /// Single
    Single,
    /// Live album
    LiveAlbum,
    /// Remix
    Remix,
    /// Bootleg
    Bootleg,
    /// Interview
    Interview,
    /// Mixtape
    Mixtape,
    /// Demo
    ///
    /// *RED only*
    Demo,
    /// Concert recording
    ///
    /// *RED only*
    ConcertRecording,
    /// DJ mix
    ///
    /// *RED only*
    DjMix,
    /// Unknown release type
    ///
    /// This is Gazelle's own "Unknown" category, not a parsing fallback.
    #[default]
    Unknown,
    /// Non-Music category with no specific release type.
    ///
    /// OPS returns `""` for these.
    NonMusic,
    /// Unrecognized integer release type
    OtherInt(i32),
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Album => write!(f, "Album"),
            Self::Soundtrack => write!(f, "Soundtrack"),
            Self::EP => write!(f, "EP"),
            Self::Anthology => write!(f, "Anthology"),
            Self::Compilation => write!(f, "Compilation"),
            Self::Single => write!(f, "Single"),
            Self::LiveAlbum => write!(f, "Live album"),
            Self::Remix => write!(f, "Remix"),
            Self::Bootleg => write!(f, "Bootleg"),
            Self::Interview => write!(f, "Interview"),
            Self::Mixtape => write!(f, "Mixtape"),
            Self::Demo => write!(f, "Demo"),
            Self::ConcertRecording => write!(f, "Concert Recording"),
            Self::DjMix => write!(f, "DJ Mix"),
            Self::Unknown => write!(f, "Unknown"),
            Self::NonMusic => write!(f, "Non-Music"),
            Self::OtherInt(n) => write!(f, "Other ({n})"),
        }
    }
}

impl ReleaseType {
    /// Integer value of the release type
    ///
    /// Returns `None` for [`NonMusic`](ReleaseType::NonMusic).
    #[must_use]
    pub fn to_int(&self) -> Option<i32> {
        match self {
            Self::Album => Some(1),
            Self::Soundtrack => Some(3),
            Self::EP => Some(5),
            Self::Anthology => Some(6),
            Self::Compilation => Some(7),
            Self::Single => Some(9),
            Self::LiveAlbum => Some(11),
            Self::Remix => Some(13),
            Self::Bootleg => Some(14),
            Self::Interview => Some(15),
            Self::Mixtape => Some(16),
            Self::Demo => Some(17),
            Self::ConcertRecording => Some(18),
            Self::DjMix => Some(19),
            Self::Unknown => Some(21),
            Self::NonMusic => None,
            Self::OtherInt(n) => Some(*n),
        }
    }

    fn from_int(n: i32) -> Self {
        match n {
            1 => Self::Album,
            3 => Self::Soundtrack,
            5 => Self::EP,
            6 => Self::Anthology,
            7 => Self::Compilation,
            9 => Self::Single,
            11 => Self::LiveAlbum,
            13 => Self::Remix,
            14 => Self::Bootleg,
            15 => Self::Interview,
            16 => Self::Mixtape,
            17 => Self::Demo,
            18 => Self::ConcertRecording,
            19 => Self::DjMix,
            21 => Self::Unknown,
            n => Self::OtherInt(n),
        }
    }
}

struct ReleaseTypeVisitor;

impl Visitor<'_> for ReleaseTypeVisitor {
    type Value = ReleaseType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or an empty string")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(ReleaseType::from_int(
            i32::try_from(value).map_err(de::Error::custom)?,
        ))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(ReleaseType::from_int(
            i32::try_from(value).map_err(de::Error::custom)?,
        ))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value.is_empty() {
            return Ok(ReleaseType::NonMusic);
        }
        Err(de::Error::invalid_value(de::Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for ReleaseType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ReleaseTypeVisitor)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_known_int() {
        let output: ReleaseType = serde_json::from_str("9").unwrap();
        assert_eq!(output, ReleaseType::Single);
    }

    #[test]
    fn deserialize_empty_string() {
        let output: ReleaseType = serde_json::from_str("\"\"").unwrap();
        assert_eq!(output, ReleaseType::NonMusic);
    }

    #[test]
    fn deserialize_unknown_int() {
        let output: ReleaseType = serde_json::from_str("99").unwrap();
        assert_eq!(output, ReleaseType::OtherInt(99));
    }

    #[test]
    fn deserialize_negative_int() {
        let output: ReleaseType = serde_json::from_str("-1").unwrap();
        assert_eq!(output, ReleaseType::OtherInt(-1));
    }

    #[test]
    fn deserialize_non_empty_string_fails() {
        let result = serde_json::from_str::<ReleaseType>("\"Album\"");
        assert!(result.is_err());
    }

    #[test]
    fn to_int_known() {
        assert_eq!(ReleaseType::Album.to_int(), Some(1));
        assert_eq!(ReleaseType::Single.to_int(), Some(9));
        assert_eq!(ReleaseType::Unknown.to_int(), Some(21));
    }

    #[test]
    fn to_int_other_int() {
        assert_eq!(ReleaseType::OtherInt(99).to_int(), Some(99));
    }

    #[test]
    fn to_int_non_music() {
        assert_eq!(ReleaseType::NonMusic.to_int(), None);
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(ReleaseType::default(), ReleaseType::Unknown);
    }

    #[test]
    fn display_known() {
        assert_eq!(ReleaseType::Album.to_string(), "Album");
        assert_eq!(ReleaseType::LiveAlbum.to_string(), "Live album");
        assert_eq!(ReleaseType::DjMix.to_string(), "DJ Mix");
    }

    #[test]
    fn display_other_int() {
        assert_eq!(ReleaseType::OtherInt(42).to_string(), "Other (42)");
    }

    #[test]
    fn display_non_music() {
        assert_eq!(ReleaseType::NonMusic.to_string(), "Non-Music");
    }
}
