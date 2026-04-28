use crate::prelude::*;

/// An entry in a collage.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collage {
    /// ID number
    pub id: u32,
    /// Collage name
    #[serde(deserialize_with = "decode_entities")]
    pub name: String,
    /// Number of torrents in the collage
    pub num_torrents: u32,
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn collage_name_decoded() {
        let json = r#"{"id":1,"name":"Rock &amp; Roll","numTorrents":42}"#;
        let collage: Collage = json_from_str(json).expect("fixture should deserialize");
        assert_eq!(collage.id, 1);
        assert_eq!(collage.name, "Rock & Roll");
        assert_eq!(collage.num_torrents, 42);
    }
}
