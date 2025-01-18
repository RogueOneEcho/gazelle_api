use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
    /// ID number
    pub id: u32,
    /// Name
    pub name: String,
}
