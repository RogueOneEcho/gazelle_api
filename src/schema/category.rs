use crate::prelude::*;

/// Category of a [`Group`].
///
/// Category IDs are:
/// - 1 indexed for Group responses returns 1-indexed category IDs
/// - 0 indexed for Upload
///
/// Use [`to_group`](Category::to_group) and [`to_upload`](Category::to_upload) to
/// get the correct ID for each context.
///
/// <https://github.com/OPSnet/Gazelle/blob/3e2f8f8ef99f654047d86ea75da166e270b85ba9/public/static/functions/upload.js#L702-L710>
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Category {
    /// Music
    #[default]
    Music,
    /// Applications
    Applications,
    /// E-Books
    EBooks,
    /// Audiobooks
    Audiobooks,
    /// E-Learning Videos
    ELearningVideos,
    /// Comedy
    Comedy,
    /// Comics
    Comics,
    /// Unrecognized category ID (1-indexed, as returned by the Group API)
    Other(i32),
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Music => write!(f, "Music"),
            Self::Applications => write!(f, "Applications"),
            Self::EBooks => write!(f, "E-Books"),
            Self::Audiobooks => write!(f, "Audiobooks"),
            Self::ELearningVideos => write!(f, "E-Learning Videos"),
            Self::Comedy => write!(f, "Comedy"),
            Self::Comics => write!(f, "Comics"),
            Self::Other(n) => write!(f, "Other ({n})"),
        }
    }
}

impl Category {
    /// 1-indexed category ID as returned by the Group API response.
    ///
    /// This is the source of truth. The database and API use 1-indexed IDs:
    /// Music = 1, Applications = 2, ..., Comics = 7.
    #[must_use]
    pub fn to_group(&self) -> i32 {
        match self {
            Self::Music => 1,
            Self::Applications => 2,
            Self::EBooks => 3,
            Self::Audiobooks => 4,
            Self::ELearningVideos => 5,
            Self::Comedy => 6,
            Self::Comics => 7,
            Self::Other(n) => *n,
        }
    }

    /// 0-indexed category ID as expected by the upload endpoint.
    ///
    /// This is `to_group() - 1`.
    #[must_use]
    pub fn to_upload(&self) -> i32 {
        self.to_group() - 1
    }

    /// Construct from a 1-indexed Group API category ID.
    #[must_use]
    fn from_group(n: i32) -> Self {
        match n {
            1 => Self::Music,
            2 => Self::Applications,
            3 => Self::EBooks,
            4 => Self::Audiobooks,
            5 => Self::ELearningVideos,
            6 => Self::Comedy,
            7 => Self::Comics,
            n => Self::Other(n),
        }
    }
}

struct CategoryVisitor;

impl Visitor<'_> for CategoryVisitor {
    type Value = Category;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("an integer")
    }

    fn visit_i64<E: DeError>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Category::from_group(
            i32::try_from(value).map_err(DeError::custom)?,
        ))
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Category::from_group(
            i32::try_from(value).map_err(DeError::custom)?,
        ))
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(CategoryVisitor)
    }
}

impl Serialize for Category {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_music() {
        let output: Category = json_from_str("1").expect("should deserialize");
        assert_eq!(output, Category::Music);
    }

    #[test]
    fn deserialize_audiobooks() {
        let output: Category = json_from_str("4").expect("should deserialize");
        assert_eq!(output, Category::Audiobooks);
    }

    #[test]
    fn deserialize_comics() {
        let output: Category = json_from_str("7").expect("should deserialize");
        assert_eq!(output, Category::Comics);
    }

    #[test]
    fn deserialize_unknown() {
        let output: Category = json_from_str("99").expect("should deserialize");
        assert_eq!(output, Category::Other(99));
    }

    #[test]
    fn to_group_known() {
        assert_eq!(Category::Music.to_group(), 1);
        assert_eq!(Category::Audiobooks.to_group(), 4);
        assert_eq!(Category::Comics.to_group(), 7);
    }

    #[test]
    fn to_upload_known() {
        assert_eq!(Category::Music.to_upload(), 0);
        assert_eq!(Category::Audiobooks.to_upload(), 3);
        assert_eq!(Category::Comics.to_upload(), 6);
    }

    #[test]
    fn to_upload_is_to_group_minus_one() {
        let categories = [
            Category::Music,
            Category::Applications,
            Category::EBooks,
            Category::Audiobooks,
            Category::ELearningVideos,
            Category::Comedy,
            Category::Comics,
        ];
        for category in &categories {
            assert_eq!(category.to_upload(), category.to_group() - 1);
        }
    }

    #[test]
    fn from_group_round_trip() {
        for n in 1..=7 {
            assert_eq!(Category::from_group(n).to_group(), n);
        }
    }

    #[test]
    fn serialize_known() {
        let output = json_to_string(&Category::Music).expect("should serialize");
        assert_eq!(output, "\"Music\"");
    }

    #[test]
    fn serialize_other() {
        let output = json_to_string(&Category::Other(99)).expect("should serialize");
        assert_eq!(output, "\"Other (99)\"");
    }

    #[test]
    fn display_known() {
        assert_eq!(Category::Music.to_string(), "Music");
        assert_eq!(Category::EBooks.to_string(), "E-Books");
        assert_eq!(Category::ELearningVideos.to_string(), "E-Learning Videos");
    }

    #[test]
    fn display_other() {
        assert_eq!(Category::Other(99).to_string(), "Other (99)");
    }
}
