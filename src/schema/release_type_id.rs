use crate::prelude::*;

/// Raw release type ID as returned by the Gazelle torrent/group API.
///
/// Integer IDs differ between RED and OPS for some release types.
/// Use [`ReleaseType::from_int_red`] or [`ReleaseType::from_int_ops`]
/// to convert to the semantic [`ReleaseType`] enum.
///
/// OPS returns `""` for non-music categories; this deserializes as `ReleaseTypeId(0)`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReleaseTypeId(pub(crate) i32);

impl ReleaseTypeId {
    /// Create a [`ReleaseTypeId`] from a raw integer.
    #[must_use]
    pub const fn from_int(id: i32) -> Self {
        Self(id)
    }
}

impl Display for ReleaseTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

struct ReleaseTypeIdVisitor;

impl Visitor<'_> for ReleaseTypeIdVisitor {
    type Value = ReleaseTypeId;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("an integer or an empty string")
    }

    fn visit_i64<E: DeError>(self, value: i64) -> Result<Self::Value, E> {
        Ok(ReleaseTypeId(
            i32::try_from(value).map_err(DeError::custom)?,
        ))
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Ok(ReleaseTypeId(
            i32::try_from(value).map_err(DeError::custom)?,
        ))
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        if value.is_empty() {
            return Ok(ReleaseTypeId(0));
        }
        Err(DeError::invalid_value(Unexpected::Str(value), &self))
    }
}

impl<'de> Deserialize<'de> for ReleaseTypeId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ReleaseTypeIdVisitor)
    }
}

impl Serialize for ReleaseTypeId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_known_int() {
        let output: ReleaseTypeId = json_from_str("9").expect("should deserialize");
        assert_eq!(output, ReleaseTypeId(9));
    }

    #[test]
    fn deserialize_unknown_int() {
        let output: ReleaseTypeId = json_from_str("99").expect("should deserialize");
        assert_eq!(output, ReleaseTypeId(99));
    }

    #[test]
    fn deserialize_empty_string() {
        let output: ReleaseTypeId = json_from_str("\"\"").expect("should deserialize");
        assert_eq!(output, ReleaseTypeId(0));
    }

    #[test]
    fn deserialize_zero() {
        let output: ReleaseTypeId = json_from_str("0").expect("should deserialize");
        assert_eq!(output, ReleaseTypeId(0));
    }

    #[test]
    fn display() {
        assert_eq!(ReleaseTypeId(9).to_string(), "9");
        assert_eq!(ReleaseTypeId(0).to_string(), "0");
    }
}
