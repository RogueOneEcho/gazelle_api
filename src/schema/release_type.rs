use crate::prelude::*;

/// Release type of a [`Group`] or [`BrowseGroup`].
///
/// Non-music categories use [`ReleaseType::NonMusic`] which are stored in the database as `0`
/// but returned by the API as `""`.
///
/// - The torrent/group endpoints return integers via [`ReleaseTypeId`]
/// - The browse endpoint returns display name strings from the `release_type` DB table
/// - DB names differ in casing from `upload.js` (e.g. `"Concert recording"` vs `"Concert Recording"`)
/// - Integer IDs differ between RED and OPS (e.g. Demo is 17 on RED, 10 on OPS)
///
/// - <https://github.com/OPSnet/Gazelle/blob/be7fae7c70028db381a5738bba6277d3b6812aa8/public/static/functions/upload.js#L580-L600>
/// - <https://github.com/OPSnet/Gazelle/blob/be7fae7c70028db381a5738bba6277d3b6812aa8/app/Json/TGroup.php#L26>
/// - <https://github.com/OPSnet/Gazelle/blob/be7fae7c70028db381a5738bba6277d3b6812aa8/app/TGroup.php#L408>
/// - <https://github.com/OPSnet/Gazelle/blob/ad946fa56a44505ff2f72fc1ce95e4c97b77ba8b/app/Json/TGroupList.php#L89>
/// - <https://github.com/OPSnet/Gazelle/blob/c7780203ace175de5fdeeace1edfac75af1294a4/misc/my-migrations/20181219213631_release_type.php>
/// - <https://github.com/OPSnet/Gazelle/blob/c7780203ace175de5fdeeace1edfac75af1294a4/misc/my-migrations/20191229132006_release_type_v2.php>
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ReleaseType {
    /// Non-Music category with no specific release type.
    ///
    /// ID: `0`
    NonMusic,
    /// Album
    ///
    /// ID: `1`
    Album,
    /// Soundtrack
    ///
    /// ID: `3`
    Soundtrack,
    /// EP
    ///
    /// ID: `5`
    EP,
    /// Anthology
    ///
    /// ID: `6`
    Anthology,
    /// Compilation
    ///
    /// ID: `7`
    Compilation,
    /// Sampler
    ///
    /// *OPS only*
    ///
    /// OPS ID: `8`
    Sampler,
    /// Single
    ///
    /// ID: `9`
    Single,
    /// Live album
    ///
    /// ID: `11`
    LiveAlbum,
    /// Split
    ///
    /// *OPS only*
    ///
    /// OPS ID: `12`
    Split,
    /// Remix
    ///
    /// ID: `13`
    Remix,
    /// Bootleg
    ///
    /// ID: `14`
    Bootleg,
    /// Interview
    ///
    /// ID: `15`
    Interview,
    /// Mixtape
    ///
    /// ID: `16`
    Mixtape,
    /// Demo
    ///
    /// RED ID: `17`
    /// OPS ID: `10`
    Demo,
    /// Concert recording
    ///
    /// ID: `18`
    ConcertRecording,
    /// DJ mix
    ///
    /// RED ID: `19`
    /// OPS ID: `17`
    DjMix,
    /// Unknown release type
    ///
    /// This is Gazelle's own "Unknown" category, not a parsing fallback.
    ///
    /// ID: `21`
    #[default]
    Unknown,
    /// Unrecognized release type display name
    Other(String),
}

impl Display for ReleaseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NonMusic => write!(f, "Non-Music"),
            Self::Album => write!(f, "Album"),
            Self::Soundtrack => write!(f, "Soundtrack"),
            Self::EP => write!(f, "EP"),
            Self::Anthology => write!(f, "Anthology"),
            Self::Compilation => write!(f, "Compilation"),
            Self::Sampler => write!(f, "Sampler"),
            Self::Single => write!(f, "Single"),
            Self::LiveAlbum => write!(f, "Live album"),
            Self::Split => write!(f, "Split"),
            Self::Remix => write!(f, "Remix"),
            Self::Bootleg => write!(f, "Bootleg"),
            Self::Interview => write!(f, "Interview"),
            Self::Mixtape => write!(f, "Mixtape"),
            Self::Demo => write!(f, "Demo"),
            Self::ConcertRecording => write!(f, "Concert Recording"),
            Self::DjMix => write!(f, "DJ Mix"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl ReleaseType {
    /// Parse a [`ReleaseType`] from its display name string.
    ///
    /// Case-insensitive. Unrecognized strings return [`Other`](ReleaseType::Other).
    /// Display names follow RED casing (e.g. `"Concert Recording"`).
    #[must_use]
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "album" => Self::Album,
            "soundtrack" => Self::Soundtrack,
            "ep" => Self::EP,
            "anthology" => Self::Anthology,
            "compilation" => Self::Compilation,
            "sampler" => Self::Sampler,
            "single" => Self::Single,
            "demo" => Self::Demo,
            "live album" => Self::LiveAlbum,
            "split" => Self::Split,
            "remix" => Self::Remix,
            "bootleg" => Self::Bootleg,
            "interview" => Self::Interview,
            "mixtape" => Self::Mixtape,
            "concert recording" => Self::ConcertRecording,
            "dj mix" => Self::DjMix,
            "unknown" => Self::Unknown,
            "non-music" | "" => Self::NonMusic,
            _ => Self::Other(s.to_owned()),
        }
    }

    /// Convert a [`ReleaseTypeId`] to a [`ReleaseType`] using RED's ID mapping.
    ///
    /// Returns [`None`] for IDs not present on RED.
    #[must_use]
    pub fn from_int_red(id: ReleaseTypeId) -> Option<Self> {
        match id.0 {
            0 => Some(Self::NonMusic),
            1 => Some(Self::Album),
            3 => Some(Self::Soundtrack),
            5 => Some(Self::EP),
            6 => Some(Self::Anthology),
            7 => Some(Self::Compilation),
            9 => Some(Self::Single),
            11 => Some(Self::LiveAlbum),
            13 => Some(Self::Remix),
            14 => Some(Self::Bootleg),
            15 => Some(Self::Interview),
            16 => Some(Self::Mixtape),
            17 => Some(Self::Demo),
            18 => Some(Self::ConcertRecording),
            19 => Some(Self::DjMix),
            21 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Convert a [`ReleaseTypeId`] to a [`ReleaseType`] using OPS's ID mapping.
    ///
    /// Returns [`None`] for IDs not present on OPS.
    /// Remaps OPS-specific IDs, then delegates shared IDs to
    /// [`from_int_red`](ReleaseType::from_int_red).
    #[must_use]
    pub fn from_int_ops(id: ReleaseTypeId) -> Option<Self> {
        match id.0 {
            8 => Some(Self::Sampler),
            10 => Some(Self::Demo),
            12 => Some(Self::Split),
            17 => Some(Self::DjMix),
            19 => None,
            _ => Self::from_int_red(id),
        }
    }

    /// Convert to a RED integer ID.
    ///
    /// Returns [`None`] for:
    /// - [`Sampler`](ReleaseType::Sampler)
    /// - [`Split`](ReleaseType::Split)
    /// - [`Other`](ReleaseType::Other)
    #[must_use]
    pub fn to_int_red(&self) -> Option<i32> {
        match self {
            Self::NonMusic => Some(0),
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
            Self::Sampler | Self::Split | Self::Other(_) => None,
        }
    }

    /// Convert to an OPS integer ID.
    ///
    /// Remaps OPS-specific variants, then delegates shared variants to
    /// [`to_int_red`](ReleaseType::to_int_red).
    /// Returns [`None`] for [`Other`](ReleaseType::Other).
    #[must_use]
    pub fn to_int_ops(&self) -> Option<i32> {
        match self {
            Self::Sampler => Some(8),
            Self::Demo => Some(10),
            Self::Split => Some(12),
            Self::DjMix => Some(17),
            _ => self.to_int_red(),
        }
    }

    /// Convert to a RED [`ReleaseTypeId`].
    ///
    /// Returns [`None`] for:
    /// - [`Sampler`](ReleaseType::Sampler)
    /// - [`Split`](ReleaseType::Split)
    /// - [`Other`](ReleaseType::Other)
    #[must_use]
    pub fn to_id_red(&self) -> Option<ReleaseTypeId> {
        self.to_int_red().map(ReleaseTypeId)
    }

    /// Convert to an OPS [`ReleaseTypeId`].
    ///
    /// Returns [`None`] for:
    /// - [`Other`](ReleaseType::Other).
    #[must_use]
    pub fn to_id_ops(&self) -> Option<ReleaseTypeId> {
        self.to_int_ops().map(ReleaseTypeId)
    }
}

impl FromStr for ReleaseType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Serialize for ReleaseType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

struct ReleaseTypeVisitor;

impl Visitor<'_> for ReleaseTypeVisitor {
    type Value = ReleaseType;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(ReleaseType::parse(value))
    }
}

impl<'de> Deserialize<'de> for ReleaseType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ReleaseTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known() {
        assert_eq!(ReleaseType::parse("Album"), ReleaseType::Album);
        assert_eq!(ReleaseType::parse("Live album"), ReleaseType::LiveAlbum);
        assert_eq!(ReleaseType::parse("DJ Mix"), ReleaseType::DjMix);
        assert_eq!(
            ReleaseType::parse("Concert Recording"),
            ReleaseType::ConcertRecording
        );
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!(ReleaseType::parse("album"), ReleaseType::Album);
        assert_eq!(ReleaseType::parse("ALBUM"), ReleaseType::Album);
        assert_eq!(ReleaseType::parse("dj mix"), ReleaseType::DjMix);
        assert_eq!(
            ReleaseType::parse("Concert recording"),
            ReleaseType::ConcertRecording
        );
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(ReleaseType::parse(""), ReleaseType::NonMusic);
    }

    #[test]
    fn parse_unrecognized() {
        assert_eq!(
            ReleaseType::parse("NotAType"),
            ReleaseType::Other("NotAType".to_owned())
        );
    }

    #[test]
    fn parse_sampler() {
        assert_eq!(ReleaseType::parse("Sampler"), ReleaseType::Sampler);
    }

    #[test]
    fn parse_split() {
        assert_eq!(ReleaseType::parse("Split"), ReleaseType::Split);
    }

    #[test]
    fn deserialize_string() {
        let output: ReleaseType = json_from_str("\"Album\"").expect("should deserialize");
        assert_eq!(output, ReleaseType::Album);
    }

    #[test]
    fn deserialize_string_case_insensitive() {
        let output: ReleaseType =
            json_from_str("\"concert recording\"").expect("should deserialize");
        assert_eq!(output, ReleaseType::ConcertRecording);
    }

    #[test]
    fn deserialize_empty_string() {
        let output: ReleaseType = json_from_str("\"\"").expect("should deserialize");
        assert_eq!(output, ReleaseType::NonMusic);
    }

    #[test]
    fn deserialize_unrecognized_string() {
        let output: ReleaseType = json_from_str("\"NewType\"").expect("should deserialize");
        assert_eq!(output, ReleaseType::Other("NewType".to_owned()));
    }

    #[test]
    fn from_int_red_known() {
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(1)),
            Some(ReleaseType::Album)
        );
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(17)),
            Some(ReleaseType::Demo)
        );
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(19)),
            Some(ReleaseType::DjMix)
        );
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(18)),
            Some(ReleaseType::ConcertRecording)
        );
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(0)),
            Some(ReleaseType::NonMusic)
        );
    }

    #[test]
    fn from_int_red_unknown() {
        assert_eq!(ReleaseType::from_int_red(ReleaseTypeId(99)), None);
        assert_eq!(ReleaseType::from_int_red(ReleaseTypeId(8)), None);
        assert_eq!(ReleaseType::from_int_red(ReleaseTypeId(10)), None);
        assert_eq!(ReleaseType::from_int_red(ReleaseTypeId(12)), None);
    }

    #[test]
    fn from_int_ops_known() {
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(1)),
            Some(ReleaseType::Album)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(10)),
            Some(ReleaseType::Demo)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(17)),
            Some(ReleaseType::DjMix)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(8)),
            Some(ReleaseType::Sampler)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(12)),
            Some(ReleaseType::Split)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(18)),
            Some(ReleaseType::ConcertRecording)
        );
    }

    #[test]
    fn from_int_ops_unknown() {
        assert_eq!(ReleaseType::from_int_ops(ReleaseTypeId(99)), None);
        assert_eq!(ReleaseType::from_int_ops(ReleaseTypeId(19)), None);
    }

    /// RED Demo=17, OPS DJ Mix=17. Verify each mapping returns the correct variant.
    #[test]
    fn from_int_ambiguous_id_17() {
        assert_eq!(
            ReleaseType::from_int_red(ReleaseTypeId(17)),
            Some(ReleaseType::Demo)
        );
        assert_eq!(
            ReleaseType::from_int_ops(ReleaseTypeId(17)),
            Some(ReleaseType::DjMix)
        );
    }

    #[test]
    fn to_int_red_known() {
        assert_eq!(ReleaseType::Album.to_int_red(), Some(1));
        assert_eq!(ReleaseType::Demo.to_int_red(), Some(17));
        assert_eq!(ReleaseType::DjMix.to_int_red(), Some(19));
        assert_eq!(ReleaseType::NonMusic.to_int_red(), Some(0));
    }

    #[test]
    fn to_int_red_ops_only() {
        assert_eq!(ReleaseType::Sampler.to_int_red(), None);
        assert_eq!(ReleaseType::Split.to_int_red(), None);
    }

    #[test]
    fn to_int_ops_known() {
        assert_eq!(ReleaseType::Album.to_int_ops(), Some(1));
        assert_eq!(ReleaseType::Demo.to_int_ops(), Some(10));
        assert_eq!(ReleaseType::DjMix.to_int_ops(), Some(17));
        assert_eq!(ReleaseType::Sampler.to_int_ops(), Some(8));
        assert_eq!(ReleaseType::Split.to_int_ops(), Some(12));
    }

    #[test]
    fn to_int_ops_red_only() {
        assert_eq!(ReleaseType::Other("Custom".to_owned()).to_int_ops(), None);
    }

    #[test]
    fn to_id_red_known() {
        assert_eq!(ReleaseType::Album.to_id_red(), Some(ReleaseTypeId(1)));
        assert_eq!(ReleaseType::Demo.to_id_red(), Some(ReleaseTypeId(17)));
        assert_eq!(
            ReleaseType::ConcertRecording.to_id_red(),
            Some(ReleaseTypeId(18))
        );
    }

    #[test]
    fn to_id_red_ops_only() {
        assert_eq!(ReleaseType::Sampler.to_id_red(), None);
        assert_eq!(ReleaseType::Split.to_id_red(), None);
        assert_eq!(ReleaseType::Other("Custom".to_owned()).to_id_red(), None);
    }

    #[test]
    fn to_id_ops_known() {
        assert_eq!(ReleaseType::Album.to_id_ops(), Some(ReleaseTypeId(1)));
        assert_eq!(ReleaseType::Demo.to_id_ops(), Some(ReleaseTypeId(10)));
        assert_eq!(ReleaseType::DjMix.to_id_ops(), Some(ReleaseTypeId(17)));
        assert_eq!(ReleaseType::Sampler.to_id_ops(), Some(ReleaseTypeId(8)));
        assert_eq!(ReleaseType::Split.to_id_ops(), Some(ReleaseTypeId(12)));
    }

    #[test]
    fn to_id_ops_other() {
        assert_eq!(ReleaseType::Other("Custom".to_owned()).to_id_ops(), None);
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(ReleaseType::default(), ReleaseType::Unknown);
    }

    #[test]
    fn display_follows_red_casing() {
        assert_eq!(ReleaseType::Album.to_string(), "Album");
        assert_eq!(ReleaseType::LiveAlbum.to_string(), "Live album");
        assert_eq!(
            ReleaseType::ConcertRecording.to_string(),
            "Concert Recording"
        );
        assert_eq!(ReleaseType::DjMix.to_string(), "DJ Mix");
        assert_eq!(ReleaseType::Sampler.to_string(), "Sampler");
        assert_eq!(ReleaseType::Split.to_string(), "Split");
    }

    #[test]
    fn display_other() {
        assert_eq!(
            ReleaseType::Other("Custom".to_owned()).to_string(),
            "Custom"
        );
    }

    #[test]
    fn display_non_music() {
        assert_eq!(ReleaseType::NonMusic.to_string(), "Non-Music");
    }

    #[test]
    fn serialize_round_trip() {
        let original = "\"Album\"";
        let release_type: ReleaseType = json_from_str(original).expect("should deserialize");
        let serialized = json_to_string(&release_type).expect("should serialize");
        assert_eq!(serialized, original);
    }

    #[test]
    fn from_str_infallible() {
        let output = ReleaseType::from_str("Album").expect("should not fail");
        assert_eq!(output, ReleaseType::Album);
    }
}
