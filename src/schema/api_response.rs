use crate::prelude::*;

/// Wrapper for Gazelle API responses
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Status.
    ///
    /// Either `success` or `failure`.
    pub status: String,
    /// API response payload.
    ///
    /// Only populated on success.
    pub response: Option<T>,
    /// Explanation of error
    #[serde(default, deserialize_with = "decode_entities_opt")]
    pub error: Option<String>,
}

#[cfg(test)]
mod decode_tests {
    use super::*;

    #[test]
    fn error_field_is_decoded() {
        let json = r#"{"status":"failure","error":"User &#039;Bob&#039; not found"}"#;
        let response: ApiResponse<()> = json_from_str(json).expect("fixture should deserialize");
        assert_eq!(response.error.as_deref(), Some("User 'Bob' not found"));
    }
}
