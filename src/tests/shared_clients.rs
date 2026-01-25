use crate::GazelleClient;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

use super::{ExampleValues, load_config};

pub type SharedClient = GazelleClient;
type SharedClients = HashMap<String, (Arc<Mutex<SharedClient>>, ExampleValues)>;

static SHARED_CLIENTS: OnceLock<SharedClients> = OnceLock::new();

pub fn get_shared_clients() -> &'static SharedClients {
    SHARED_CLIENTS.get_or_init(|| {
        load_config()
            .into_iter()
            .map(|(name, config)| {
                let client = Arc::new(Mutex::new(GazelleClient::from(config.client)));
                (name, (client, config.examples))
            })
            .collect()
    })
}
