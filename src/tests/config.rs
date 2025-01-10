use crate::GazelleClientOptions;
use rogue_config::{OptionsProvider, YamlOptionsProvider};
use rogue_logging::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct ExampleValues {
    pub torrent: u32,
    pub group: u32,
    pub user: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    pub clients: HashMap<String, GazelleClientOptions>,
    pub examples: HashMap<String, ExampleValues>,
}

#[derive(Deserialize, Serialize)]
pub struct ConfigSet {
    pub client: GazelleClientOptions,
    pub examples: ExampleValues,
}

pub fn load_config() -> Result<HashMap<String, ConfigSet>, Error> {
    let config: ConfigFile = YamlOptionsProvider::get()?;
    let vec = config
        .clients
        .into_iter()
        .map(|(name, client)| {
            let examples = config
                .examples
                .get(&name)
                .cloned()
                .expect("examples should have key");
            (name, ConfigSet { client, examples })
        })
        .collect();
    Ok(vec)
}
