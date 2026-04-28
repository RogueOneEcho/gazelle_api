use crate::prelude::*;

/// Freeleech status of a torrent.
///
/// - *RED*: returns a bool; [`LeechType::Bool`]`(true)` is ambiguous between free and neutral
/// - *OPS*: returns a string label that maps to a [`LeechKind`]
#[derive(Clone, Debug, PartialEq)]
pub enum LeechType {
    /// Bool representation, ambiguous between free and neutral.
    ///
    /// *RED only*
    Bool(bool),
    /// Labeled representation with unambiguous semantics.
    ///
    /// *OPS only*
    Kind(LeechKind),
}

/// Freeleech kind.
///
/// *OPS only*
#[derive(Clone, Debug, PartialEq)]
pub enum LeechKind {
    /// Not freeleech
    Normal,
    /// No download cost
    Free,
    /// No upload credit and no download cost
    Neutral,
}

/// Deserialize [`LeechType`] from a bool, a string label, or null.
pub(crate) fn deserialize_leech_type<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<LeechType>, D::Error> {
    struct LeechTypeVisitor;

    impl Visitor<'_> for LeechTypeVisitor {
        type Value = Option<LeechType>;

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("a bool, a leech type string, or null")
        }

        fn visit_bool<E: DeError>(self, value: bool) -> Result<Self::Value, E> {
            Ok(Some(LeechType::Bool(value)))
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            let kind = match value {
                "Normal" | "0" => LeechKind::Normal,
                "Freeleech" | "1" => LeechKind::Free,
                "Neutral Leech" | "2" => LeechKind::Neutral,
                other => {
                    return Err(DeError::invalid_value(
                        Unexpected::Str(other),
                        &"Normal, Freeleech, Neutral Leech, 0, 1, or 2",
                    ));
                }
            };
            Ok(Some(LeechType::Kind(kind)))
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(LeechTypeVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deser(json: &str) -> Option<LeechType> {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default, deserialize_with = "deserialize_leech_type")]
            value: Option<LeechType>,
        }
        let w: Wrapper = json_from_str(json).expect("fixture should deserialize");
        w.value
    }

    #[test]
    fn bool_false() {
        let output = deser(r#"{"value": false}"#);
        assert_eq!(output, Some(LeechType::Bool(false)));
    }

    #[test]
    fn bool_true() {
        let output = deser(r#"{"value": true}"#);
        assert_eq!(output, Some(LeechType::Bool(true)));
    }

    #[test]
    fn string_normal() {
        let output = deser(r#"{"value": "Normal"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Normal)));
    }

    #[test]
    fn string_freeleech() {
        let output = deser(r#"{"value": "Freeleech"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Free)));
    }

    #[test]
    fn string_neutral_leech() {
        let output = deser(r#"{"value": "Neutral Leech"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Neutral)));
    }

    #[test]
    fn backing_value_0() {
        let output = deser(r#"{"value": "0"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Normal)));
    }

    #[test]
    fn backing_value_1() {
        let output = deser(r#"{"value": "1"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Free)));
    }

    #[test]
    fn backing_value_2() {
        let output = deser(r#"{"value": "2"}"#);
        assert_eq!(output, Some(LeechType::Kind(LeechKind::Neutral)));
    }

    #[test]
    fn null_value() {
        let output = deser(r#"{"value": null}"#);
        assert_eq!(output, None);
    }

    #[test]
    fn absent_value() {
        let output = deser(r"{}");
        assert_eq!(output, None);
    }
}
