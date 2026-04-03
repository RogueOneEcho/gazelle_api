use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// Audio format of a [`Torrent`].
///
/// - `OggVorbis` is *OPS only*
/// - `DSD` is *RED only*
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Format {
    /// MP3
    MP3,
    /// FLAC
    #[default]
    FLAC,
    /// AAC
    AAC,
    /// AC3
    AC3,
    /// DTS
    DTS,
    /// Ogg Vorbis
    ///
    /// *OPS only*
    OggVorbis,
    /// DSD
    ///
    /// *RED only*
    DSD,
    /// Unrecognized format
    Other(String),
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MP3 => write!(f, "MP3"),
            Self::FLAC => write!(f, "FLAC"),
            Self::AAC => write!(f, "AAC"),
            Self::AC3 => write!(f, "AC3"),
            Self::DTS => write!(f, "DTS"),
            Self::OggVorbis => write!(f, "Ogg Vorbis"),
            Self::DSD => write!(f, "DSD"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Format {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "mp3" => Self::MP3,
            "flac" => Self::FLAC,
            "aac" => Self::AAC,
            "ac3" => Self::AC3,
            "dts" => Self::DTS,
            "ogg vorbis" => Self::OggVorbis,
            "dsd" => Self::DSD,
            _ => Self::Other(s.to_owned()),
        }
    }
}

struct FormatVisitor;

impl Visitor<'_> for FormatVisitor {
    type Value = Format;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Format::from_str(value))
    }
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(FormatVisitor)
    }
}

impl Serialize for Format {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_known() {
        let output: Format = serde_json::from_str("\"FLAC\"").unwrap();
        assert_eq!(output, Format::FLAC);
    }

    #[test]
    fn deserialize_ogg_vorbis() {
        let output: Format = serde_json::from_str("\"Ogg Vorbis\"").unwrap();
        assert_eq!(output, Format::OggVorbis);
    }

    #[test]
    fn deserialize_dsd() {
        let output: Format = serde_json::from_str("\"DSD\"").unwrap();
        assert_eq!(output, Format::DSD);
    }

    #[test]
    fn deserialize_unknown() {
        let output: Format = serde_json::from_str("\"Opus\"").unwrap();
        assert_eq!(output, Format::Other("Opus".to_owned()));
    }

    #[test]
    fn deserialize_empty_string() {
        let output: Format = serde_json::from_str("\"\"").unwrap();
        assert_eq!(output, Format::Other(String::new()));
    }

    #[test]
    fn serialize_known() {
        let output = serde_json::to_string(&Format::FLAC).unwrap();
        assert_eq!(output, "\"FLAC\"");
    }

    #[test]
    fn serialize_ogg_vorbis() {
        let output = serde_json::to_string(&Format::OggVorbis).unwrap();
        assert_eq!(output, "\"Ogg Vorbis\"");
    }

    #[test]
    fn serialize_round_trip() {
        let original = "\"Ogg Vorbis\"";
        let format: Format = serde_json::from_str(original).unwrap();
        let serialized = serde_json::to_string(&format).unwrap();
        assert_eq!(serialized, original);
    }

    #[test]
    fn display_known() {
        assert_eq!(Format::MP3.to_string(), "MP3");
        assert_eq!(Format::OggVorbis.to_string(), "Ogg Vorbis");
    }

    #[test]
    fn display_other() {
        assert_eq!(Format::Other("Opus".to_owned()).to_string(), "Opus");
    }
}
