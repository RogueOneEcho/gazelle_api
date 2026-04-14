use crate::prelude::*;

/// Media type of a [`Torrent`].
///
/// - `BluRay` is the RED value (`"Blu-Ray"`)
/// - `BD` is the OPS value (`"BD"`)
/// - Both represent optical disc media
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Media {
    /// CD
    CD,
    /// DVD
    DVD,
    /// Vinyl
    Vinyl,
    /// Soundboard
    Soundboard,
    /// SACD
    SACD,
    /// DAT
    DAT,
    /// Cassette
    Cassette,
    /// WEB
    #[default]
    WEB,
    /// Blu-Ray
    ///
    /// *RED only*
    BluRay,
    /// BD (Blu-ray Disc)
    ///
    /// *OPS only*
    BD,
    /// Unrecognized media type
    Other(String),
}

impl Display for Media {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::CD => write!(f, "CD"),
            Self::DVD => write!(f, "DVD"),
            Self::Vinyl => write!(f, "Vinyl"),
            Self::Soundboard => write!(f, "Soundboard"),
            Self::SACD => write!(f, "SACD"),
            Self::DAT => write!(f, "DAT"),
            Self::Cassette => write!(f, "Cassette"),
            Self::WEB => write!(f, "WEB"),
            Self::BluRay => write!(f, "Blu-Ray"),
            Self::BD => write!(f, "BD"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Media {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "cd" => Self::CD,
            "dvd" => Self::DVD,
            "vinyl" => Self::Vinyl,
            "soundboard" => Self::Soundboard,
            "sacd" => Self::SACD,
            "dat" => Self::DAT,
            "cassette" => Self::Cassette,
            "web" => Self::WEB,
            "blu-ray" => Self::BluRay,
            "bd" => Self::BD,
            _ => Self::Other(s.to_owned()),
        }
    }
}

struct MediaVisitor;

impl Visitor<'_> for MediaVisitor {
    type Value = Media;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Media::from_str(value))
    }
}

impl<'de> Deserialize<'de> for Media {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(MediaVisitor)
    }
}

impl Serialize for Media {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_known() {
        let output: Media = serde_json::from_str("\"CD\"").expect("should deserialize");
        assert_eq!(output, Media::CD);
    }

    #[test]
    fn deserialize_blu_ray() {
        let output: Media = serde_json::from_str("\"Blu-Ray\"").expect("should deserialize");
        assert_eq!(output, Media::BluRay);
    }

    #[test]
    fn deserialize_bd() {
        let output: Media = serde_json::from_str("\"BD\"").expect("should deserialize");
        assert_eq!(output, Media::BD);
    }

    #[test]
    fn deserialize_unknown() {
        let output: Media = serde_json::from_str("\"MiniDisc\"").expect("should deserialize");
        assert_eq!(output, Media::Other("MiniDisc".to_owned()));
    }

    #[test]
    fn deserialize_empty_string() {
        let output: Media = serde_json::from_str("\"\"").expect("should deserialize");
        assert_eq!(output, Media::Other(String::new()));
    }

    #[test]
    fn serialize_known() {
        let output = serde_json::to_string(&Media::WEB).expect("should serialize");
        assert_eq!(output, "\"WEB\"");
    }

    #[test]
    fn serialize_blu_ray() {
        let output = serde_json::to_string(&Media::BluRay).expect("should serialize");
        assert_eq!(output, "\"Blu-Ray\"");
    }

    #[test]
    fn serialize_other() {
        let output =
            serde_json::to_string(&Media::Other("MiniDisc".to_owned())).expect("should serialize");
        assert_eq!(output, "\"MiniDisc\"");
    }

    #[test]
    fn serialize_round_trip() {
        let original = "\"Cassette\"";
        let media: Media = serde_json::from_str(original).expect("should deserialize");
        let serialized = serde_json::to_string(&media).expect("should serialize");
        assert_eq!(serialized, original);
    }

    #[test]
    fn display_known() {
        assert_eq!(Media::CD.to_string(), "CD");
        assert_eq!(Media::BluRay.to_string(), "Blu-Ray");
        assert_eq!(Media::BD.to_string(), "BD");
    }

    #[test]
    fn display_other() {
        assert_eq!(Media::Other("MiniDisc".to_owned()).to_string(), "MiniDisc");
    }
}
