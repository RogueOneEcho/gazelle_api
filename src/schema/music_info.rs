use crate::Artist;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicInfo {
    pub arranger: Option<Vec<Artist>>,
    pub artists: Vec<Artist>,
    pub composers: Vec<Artist>,
    pub conductor: Vec<Artist>,
    pub dj: Vec<Artist>,
    pub producer: Vec<Artist>,
    pub remixed_by: Vec<Artist>,
    pub with: Vec<Artist>,
}
