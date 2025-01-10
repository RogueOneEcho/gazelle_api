use crate::Credit;
use serde::{Deserialize, Serialize};

/// Release credits
/// Artists, composer etc
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credits {
    /// Arrangers
    /// *OPS only*
    pub arranger: Option<Vec<Credit>>,
    /// Artists
    pub artists: Vec<Credit>,
    /// Composers
    /// Typically present for classical works
    pub composers: Vec<Credit>,
    /// Conductors
    /// Typically present for classical works
    pub conductor: Vec<Credit>,
    /// DJs
    pub dj: Vec<Credit>,
    /// Producers
    pub producer: Vec<Credit>,
    /// Remix artist
    pub remixed_by: Vec<Credit>,
    /// Featured artists
    pub with: Vec<Credit>,
}
