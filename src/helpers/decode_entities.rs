use crate::prelude::*;
use html_escape::decode_html_entities;

pub(crate) fn decode_entities<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(decode_html_entities(&raw).into_owned())
}

pub(crate) fn decode_entities_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<String>::deserialize(deserializer)?;
    Ok(raw.map(|s| decode_html_entities(&s).into_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct PlainWrapper {
        #[serde(deserialize_with = "decode_entities")]
        value: String,
    }

    #[derive(Deserialize)]
    struct OptWrapper {
        #[serde(deserialize_with = "decode_entities_opt")]
        value: Option<String>,
    }

    fn decode_plain(json: &str) -> String {
        json_from_str::<PlainWrapper>(json)
            .expect("test JSON should deserialize")
            .value
    }

    fn decode_opt(json: &str) -> Option<String> {
        json_from_str::<OptWrapper>(json)
            .expect("test JSON should deserialize")
            .value
    }

    #[test]
    fn decode_entities_common() {
        assert_eq!(decode_plain(r#"{"value":"&amp;"}"#), "&");
        assert_eq!(decode_plain(r#"{"value":"&lt;"}"#), "<");
        assert_eq!(decode_plain(r#"{"value":"&gt;"}"#), ">");
        assert_eq!(decode_plain(r#"{"value":"&quot;"}"#), "\"");
        assert_eq!(decode_plain(r#"{"value":"&#039;"}"#), "'");
    }

    #[test]
    fn decode_entities_plain_passthrough() {
        assert_eq!(decode_plain(r#"{"value":"hello world"}"#), "hello world");
    }

    #[test]
    fn decode_entities_empty() {
        assert_eq!(decode_plain(r#"{"value":""}"#), "");
    }

    #[test]
    fn decode_entities_chinese() {
        assert_eq!(
            decode_plain(r#"{"value":"&#26085;&#26412;&#35486;"}"#),
            "日本語"
        );
    }

    #[test]
    fn decode_entities_japanese_mixed() {
        assert_eq!(
            decode_plain(r#"{"value":"&#26481;&#20140;&#12399;&#29105;&#12356;"}"#),
            "東京は熱い"
        );
    }

    #[test]
    fn decode_entities_korean() {
        assert_eq!(
            decode_plain(r#"{"value":"&#54620;&#44397;&#50612;"}"#),
            "한국어"
        );
    }

    #[test]
    fn decode_entities_ukrainian() {
        assert_eq!(
            decode_plain(r#"{"value":"&#1059;&#1082;&#1088;&#1072;&#1111;&#1085;&#1072;"}"#),
            "Україна"
        );
    }

    #[test]
    fn decode_entities_emoji_decimal() {
        assert_eq!(decode_plain(r#"{"value":"&#128512;&#127881;"}"#), "😀🎉");
    }

    #[test]
    fn decode_entities_emoji_hex() {
        assert_eq!(decode_plain(r#"{"value":"&#x1F600;"}"#), "😀");
    }

    #[test]
    fn decode_entities_mixed_run() {
        assert_eq!(
            decode_plain(r#"{"value":"Album &#8211; &#26085;&#26412;"}"#),
            "Album – 日本"
        );
    }

    #[test]
    fn decode_entities_opt_null() {
        assert_eq!(decode_opt(r#"{"value":null}"#), None);
    }

    #[test]
    fn decode_entities_opt_some() {
        assert_eq!(decode_opt(r#"{"value":"&amp;"}"#), Some("&".to_owned()));
    }
}
