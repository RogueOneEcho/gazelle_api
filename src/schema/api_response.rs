use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub status: String,
    pub response: Option<T>,
    pub error: Option<String>,
    pub data: Option<String>,
}

impl<T> Display for ApiResponse<T> {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let status = &self.status.clone();
        let response = if self.response.is_some() {
            "Some"
        } else {
            "None"
        };
        let data = &self.data.clone().unwrap_or("None".to_owned());
        let error = &self.error.clone().unwrap_or("None".to_owned());
        write!(
            formatter,
            "Status: {status}\nResponse: {response}\nError: {error}\nData: {data}"
        )
    }
}
