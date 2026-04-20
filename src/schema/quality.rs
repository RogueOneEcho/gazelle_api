use crate::prelude::*;

/// Audio quality of a [`Torrent`].
///
/// Referred to as `encoding` in the torrent API response and `bitrate` in the upload form.
/// Values are a mix of fixed bitrates, VBR quality presets, and lossless indicators.
///
/// - `_160`, `_128`, `_96`, `_64`, `Q8x` are *OPS only*
/// - `DSD64`, `DSD128`, `DSD256`, `DSD512` are *RED only*
/// - The Gazelle UI has an "Other" option with freeform text input, so the API can return
///   arbitrary strings
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Quality {
    /// Lossless
    #[default]
    Lossless,
    /// 24bit Lossless
    Lossless24,
    /// V0 (VBR)
    V0,
    /// V1 (VBR)
    V1,
    /// V2 (VBR)
    V2,
    /// 320 kbps
    _320,
    /// 256 kbps
    _256,
    /// 192 kbps
    _192,
    /// 160 kbps
    ///
    /// *OPS only*
    _160,
    /// 128 kbps
    ///
    /// *OPS only*
    _128,
    /// 96 kbps
    ///
    /// *OPS only*
    _96,
    /// 64 kbps
    ///
    /// *OPS only*
    _64,
    /// APS (VBR)
    APS,
    /// APX (VBR)
    APX,
    /// q8.x (VBR)
    ///
    /// *OPS only*
    Q8x,
    /// DSD64
    ///
    /// *RED only*
    DSD64,
    /// DSD128
    ///
    /// *RED only*
    DSD128,
    /// DSD256
    ///
    /// *RED only*
    DSD256,
    /// DSD512
    ///
    /// *RED only*
    DSD512,
    /// Unrecognized quality
    Other(String),
}

impl Display for Quality {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Lossless => write!(f, "Lossless"),
            Self::Lossless24 => write!(f, "24bit Lossless"),
            Self::V0 => write!(f, "V0 (VBR)"),
            Self::V1 => write!(f, "V1 (VBR)"),
            Self::V2 => write!(f, "V2 (VBR)"),
            Self::_320 => write!(f, "320"),
            Self::_256 => write!(f, "256"),
            Self::_192 => write!(f, "192"),
            Self::_160 => write!(f, "160"),
            Self::_128 => write!(f, "128"),
            Self::_96 => write!(f, "96"),
            Self::_64 => write!(f, "64"),
            Self::APS => write!(f, "APS (VBR)"),
            Self::APX => write!(f, "APX (VBR)"),
            Self::Q8x => write!(f, "q8.x (VBR)"),
            Self::DSD64 => write!(f, "DSD64"),
            Self::DSD128 => write!(f, "DSD128"),
            Self::DSD256 => write!(f, "DSD256"),
            Self::DSD512 => write!(f, "DSD512"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Quality {
    fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "lossless" => Self::Lossless,
            "24bit lossless" => Self::Lossless24,
            "v0 (vbr)" => Self::V0,
            "v1 (vbr)" => Self::V1,
            "v2 (vbr)" => Self::V2,
            "320" => Self::_320,
            "256" => Self::_256,
            "192" => Self::_192,
            "160" => Self::_160,
            "128" => Self::_128,
            "96" => Self::_96,
            "64" => Self::_64,
            "aps (vbr)" => Self::APS,
            "apx (vbr)" => Self::APX,
            "q8.x (vbr)" => Self::Q8x,
            "dsd64" => Self::DSD64,
            "dsd128" => Self::DSD128,
            "dsd256" => Self::DSD256,
            "dsd512" => Self::DSD512,
            _ => Self::Other(s.to_owned()),
        }
    }
}

struct QualityVisitor;

impl Visitor<'_> for QualityVisitor {
    type Value = Quality;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Quality::from_str(value))
    }
}

impl<'de> Deserialize<'de> for Quality {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(QualityVisitor)
    }
}

impl Serialize for Quality {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_lossless() {
        let output: Quality = json_from_str("\"Lossless\"").expect("should deserialize");
        assert_eq!(output, Quality::Lossless);
    }

    #[test]
    fn deserialize_24bit_lossless() {
        let output: Quality = json_from_str("\"24bit Lossless\"").expect("should deserialize");
        assert_eq!(output, Quality::Lossless24);
    }

    #[test]
    fn deserialize_vbr() {
        let output: Quality = json_from_str("\"V0 (VBR)\"").expect("should deserialize");
        assert_eq!(output, Quality::V0);
    }

    #[test]
    fn deserialize_numeric() {
        let output: Quality = json_from_str("\"320\"").expect("should deserialize");
        assert_eq!(output, Quality::_320);
    }

    #[test]
    fn deserialize_q8x() {
        let output: Quality = json_from_str("\"q8.x (VBR)\"").expect("should deserialize");
        assert_eq!(output, Quality::Q8x);
    }

    #[test]
    fn deserialize_dsd() {
        let output: Quality = json_from_str("\"DSD128\"").expect("should deserialize");
        assert_eq!(output, Quality::DSD128);
    }

    #[test]
    fn deserialize_unknown() {
        let output: Quality = json_from_str("\"Custom 448\"").expect("should deserialize");
        assert_eq!(output, Quality::Other("Custom 448".to_owned()));
    }

    #[test]
    fn deserialize_empty_string() {
        let output: Quality = json_from_str("\"\"").expect("should deserialize");
        assert_eq!(output, Quality::Other(String::new()));
    }

    #[test]
    fn serialize_lossless() {
        let output = json_to_string(&Quality::Lossless).expect("should serialize");
        assert_eq!(output, "\"Lossless\"");
    }

    #[test]
    fn serialize_vbr() {
        let output = json_to_string(&Quality::V0).expect("should serialize");
        assert_eq!(output, "\"V0 (VBR)\"");
    }

    #[test]
    fn serialize_numeric() {
        let output = json_to_string(&Quality::_320).expect("should serialize");
        assert_eq!(output, "\"320\"");
    }

    #[test]
    fn serialize_round_trip() {
        let original = "\"24bit Lossless\"";
        let quality: Quality = json_from_str(original).expect("should deserialize");
        let serialized = json_to_string(&quality).expect("should serialize");
        assert_eq!(serialized, original);
    }

    #[test]
    fn display_vbr() {
        assert_eq!(Quality::V0.to_string(), "V0 (VBR)");
        assert_eq!(Quality::APS.to_string(), "APS (VBR)");
        assert_eq!(Quality::Q8x.to_string(), "q8.x (VBR)");
    }

    #[test]
    fn display_numeric() {
        assert_eq!(Quality::_320.to_string(), "320");
        assert_eq!(Quality::_192.to_string(), "192");
    }

    #[test]
    fn display_other() {
        assert_eq!(Quality::Other("Custom".to_owned()).to_string(), "Custom");
    }
}
