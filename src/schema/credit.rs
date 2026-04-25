use crate::prelude::*;

/// A credited artist or contributor on a release
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
    /// ID number
    pub id: u32,
    /// Name
    #[serde(deserialize_with = "decode_entities")]
    pub name: String,
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn credit_name_decoded() {
        let json = r#"{"id":1,"name":"DJ &amp; MC"}"#;
        let credit: Credit = json_from_str(json).expect("fixture should deserialize");
        assert_eq!(credit.name, "DJ & MC");
    }
}
