use crate::{GazelleClient, GazelleClientFactory};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GazelleClientOptions {
    pub name: String,
    pub key: String,
    pub url: String,
}

impl GazelleClientOptions {
    pub(crate) fn get_client(&self) -> GazelleClient {
        let factory = GazelleClientFactory {
            key: self.key.clone(),
            url: self.url.clone(),
            user_agent: "gazelle_api.rs".to_owned(),
        };
        factory.create()
    }
}
